use std::path::Path;
use std::process::Command;

use crate::config::ADMIN_TASK_DEFAULT_NAME;
use crate::models::ApiResult;

const SINGLE_INSTANCE_MUTEX_NAME: &str = "Local\\com.mfdust.kvrandom.single-instance";
pub(crate) struct SingleInstanceGuard(windows::Win32::Foundation::HANDLE);

#[cfg(target_os = "windows")]
unsafe impl Send for SingleInstanceGuard {}

#[cfg(target_os = "windows")]
unsafe impl Sync for SingleInstanceGuard {}

#[cfg(target_os = "windows")]
impl Drop for SingleInstanceGuard {
    fn drop(&mut self) {
        unsafe {
            let _ = windows::Win32::Foundation::CloseHandle(self.0);
        }
    }
}

#[cfg(target_os = "windows")]
pub(crate) fn acquire_single_instance_guard() -> Result<Option<SingleInstanceGuard>, String> {
    use windows::core::PCWSTR;
    use windows::Win32::Foundation::{CloseHandle, GetLastError, ERROR_ALREADY_EXISTS};
    use windows::Win32::System::Threading::CreateMutexW;

    let name = SINGLE_INSTANCE_MUTEX_NAME
        .encode_utf16()
        .chain(std::iter::once(0))
        .collect::<Vec<_>>();

    unsafe {
        let handle = CreateMutexW(None, true, PCWSTR(name.as_ptr()))
            .map_err(|error| format!("创建单实例锁失败: {error}"))?;
        if GetLastError() == ERROR_ALREADY_EXISTS {
            let _ = CloseHandle(handle);
            return Ok(None);
        }
        Ok(Some(SingleInstanceGuard(handle)))
    }
}

#[cfg(not(target_os = "windows"))]
pub(crate) struct SingleInstanceGuard;

#[cfg(not(target_os = "windows"))]
pub(crate) fn acquire_single_instance_guard() -> Result<Option<SingleInstanceGuard>, String> {
    Ok(Some(SingleInstanceGuard))
}

fn quote_for_powershell(text: &str) -> String {
    text.replace('\'', "''")
}

#[cfg(target_os = "windows")]
pub(crate) fn is_process_elevated() -> bool {
    use windows::Win32::Foundation::CloseHandle;
    use windows::Win32::Security::{
        GetTokenInformation, TokenElevation, TOKEN_ELEVATION, TOKEN_QUERY,
    };
    use windows::Win32::System::Threading::{GetCurrentProcess, OpenProcessToken};

    unsafe {
        let mut token = Default::default();
        if OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut token).is_err() {
            return false;
        }

        let mut elevation = TOKEN_ELEVATION::default();
        let mut returned_len = 0;
        let ok = GetTokenInformation(
            token,
            TokenElevation,
            Some(&mut elevation as *mut _ as *mut _),
            std::mem::size_of::<TOKEN_ELEVATION>() as u32,
            &mut returned_len,
        )
        .is_ok();
        let _ = CloseHandle(token);

        ok && elevation.TokenIsElevated != 0
    }
}

#[cfg(not(target_os = "windows"))]
pub(crate) fn is_process_elevated() -> bool {
    false
}

#[cfg(target_os = "windows")]
pub(crate) fn request_admin_relaunch() -> ApiResult {
    let exe_path = std::env::current_exe().unwrap_or_default();
    let exe_text = exe_path.to_string_lossy();
    let args = std::env::args()
        .skip(1)
        .map(|arg| format!("\"{}\"", arg.replace('"', "\\\"")))
        .collect::<Vec<_>>()
        .join(" ");
    let command = if args.is_empty() {
        format!(
            "Start-Process -FilePath '{}' -Verb RunAs",
            quote_for_powershell(&exe_text)
        )
    } else {
        format!(
            "Start-Process -FilePath '{}' -ArgumentList '{}' -Verb RunAs",
            quote_for_powershell(&exe_text),
            quote_for_powershell(&args)
        )
    };

    match Command::new("powershell")
        .args(["-NoProfile", "-Command", &command])
        .status()
    {
        Ok(status) if status.success() => ApiResult {
            ok: true,
            message: "已请求管理员权限，即将重新启动。".to_string(),
            detail: None,
            restart_required: None,
        },
        Ok(status) => ApiResult {
            ok: false,
            message: "管理员权限请求失败或被取消。".to_string(),
            detail: Some(format!("exit code: {:?}", status.code())),
            restart_required: None,
        },
        Err(error) => ApiResult {
            ok: false,
            message: "管理员权限请求失败或被取消。".to_string(),
            detail: Some(error.to_string()),
            restart_required: None,
        },
    }
}

#[cfg(not(target_os = "windows"))]
pub(crate) fn request_admin_relaunch() -> ApiResult {
    ApiResult {
        ok: false,
        message: "当前系统不支持管理员提升。".to_string(),
        detail: None,
        restart_required: None,
    }
}

#[cfg(target_os = "windows")]
pub(crate) fn create_admin_startup_task_impl(task_name: &str, exe_path: &str) -> ApiResult {
    if exe_path.trim().is_empty() || !Path::new(exe_path).exists() {
        return ApiResult {
            ok: false,
            message: "可执行文件路径无效或不存在。".to_string(),
            detail: None,
            restart_required: None,
        };
    }

    let safe_task_name = if task_name.trim().is_empty() {
        ADMIN_TASK_DEFAULT_NAME
    } else {
        task_name.trim()
    };
    let user_name = std::env::var("USERNAME").unwrap_or_default();
    let mut task_args = vec![
        "/Create".to_string(),
        "/F".to_string(),
        "/RL".to_string(),
        "HIGHEST".to_string(),
        "/SC".to_string(),
        "ONLOGON".to_string(),
        "/TN".to_string(),
        safe_task_name.to_string(),
        "/TR".to_string(),
        format!("\"{exe_path}\""),
    ];
    if !user_name.is_empty() {
        task_args.push("/RU".to_string());
        task_args.push(user_name);
    }

    let result = if is_process_elevated() {
        Command::new("schtasks").args(&task_args).status()
    } else {
        let ps_args = task_args
            .iter()
            .map(|arg| format!("\"{}\"", arg.replace('"', "\\\"")))
            .collect::<Vec<_>>()
            .join(" ");
        let command = format!(
            "Start-Process -FilePath 'schtasks.exe' -ArgumentList '{}' -Verb RunAs -Wait",
            quote_for_powershell(&ps_args)
        );
        Command::new("powershell")
            .args(["-NoProfile", "-Command", &command])
            .status()
    };

    match result {
        Ok(status) if status.success() => ApiResult {
            ok: true,
            message: "计划任务已创建或更新。".to_string(),
            detail: None,
            restart_required: None,
        },
        Ok(status) => ApiResult {
            ok: false,
            message: "计划任务创建失败或被取消。".to_string(),
            detail: Some(format!("exit code: {:?}", status.code())),
            restart_required: None,
        },
        Err(error) => ApiResult {
            ok: false,
            message: "计划任务创建失败或被取消。".to_string(),
            detail: Some(error.to_string()),
            restart_required: None,
        },
    }
}

#[cfg(not(target_os = "windows"))]
pub(crate) fn create_admin_startup_task_impl(_task_name: &str, _exe_path: &str) -> ApiResult {
    ApiResult {
        ok: false,
        message: "仅支持 Windows 计划任务。".to_string(),
        detail: None,
        restart_required: None,
    }
}
