use std::fs;
use std::hash::{Hash, Hasher};
use tauri::{AppHandle, Emitter, Manager, PhysicalPosition, Position, WebviewWindow};

use crate::admin::{create_admin_startup_task_impl, is_process_elevated, request_admin_relaunch};
use crate::audio::AudioCommand;
use crate::config::{
    normalize_config, parse_student_list_text_impl, save_config, AppConfig, FloatingButtonConfig,
    PickCountDialogConfig, PickResultDialogConfig, Student, StudentListParseResult,
    ADMIN_TASK_DEFAULT_NAME,
};
use crate::models::{ApiResult, AppInfo, PickResultResetPayload, PickedStudent, UpdateResult};
use crate::picker::{build_weighted_pool, pick_students_with_repeat, pick_students_without_repeat};
use crate::state::{cached_config, push_log, AppState, DragSession, LogEntry};
use crate::update::check_update_from_main;
use crate::utils::clamp_i32;
use crate::windows::{
    apply_floating_window_config, create_floating_window, create_pick_count_window,
    hide_floating_window, open_pick_result_window, persist_floating_position, show_floating_window,
};
#[tauri::command]
pub(crate) fn get_floating_button_config(
    state: tauri::State<'_, AppState>,
) -> Result<FloatingButtonConfig, String> {
    Ok(state
        .inner
        .lock()
        .map_err(|_| "状态锁定失败".to_string())?
        .config
        .floating_button
        .clone())
}

#[tauri::command]
pub(crate) fn floating_button_clicked(
    app: AppHandle,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    open_pick_count(app, state)
}

#[tauri::command]
pub(crate) fn floating_button_drag_start(
    window: WebviewWindow,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    let position = window.outer_position().map_err(|error| error.to_string())?;
    state
        .inner
        .lock()
        .map_err(|_| "状态锁定失败".to_string())?
        .drag_session = Some(DragSession {
        start_x: position.x,
        start_y: position.y,
        last_x: position.x,
        last_y: position.y,
    });
    Ok(())
}

#[tauri::command]
pub(crate) fn floating_button_drag_move(
    window: WebviewWindow,
    state: tauri::State<'_, AppState>,
    dx: f64,
    dy: f64,
) -> Result<(), String> {
    let mut guard = state.inner.lock().map_err(|_| "状态锁定失败".to_string())?;
    let Some(session) = &mut guard.drag_session else {
        return Ok(());
    };
    let next_x = session.start_x + dx.round() as i32;
    let next_y = session.start_y + dy.round() as i32;
    if (next_x - session.last_x).abs() < 2 && (next_y - session.last_y).abs() < 2 {
        return Ok(());
    }
    session.last_x = next_x;
    session.last_y = next_y;
    drop(guard);
    window
        .set_position(Position::Physical(PhysicalPosition {
            x: next_x,
            y: next_y,
        }))
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub(crate) fn floating_button_drag_end(
    app: AppHandle,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    state
        .inner
        .lock()
        .map_err(|_| "状态锁定失败".to_string())?
        .drag_session = None;
    persist_floating_position(&app, &state);
    Ok(())
}

#[tauri::command]
pub(crate) fn floating_button_set_ignore_mouse(
    window: WebviewWindow,
    _ignore: bool,
) -> Result<(), String> {
    // Electron can ignore transparent pixels while forwarding mouse movement back to
    // the renderer. Tauri's cursor ignore API does not provide that forwarding mode,
    // so enabling it here makes the floating button impossible to click again.
    window
        .set_ignore_cursor_events(false)
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub(crate) fn get_pick_count_config(
    state: tauri::State<'_, AppState>,
) -> Result<PickCountDialogConfig, String> {
    Ok(state
        .inner
        .lock()
        .map_err(|_| "状态锁定失败".to_string())?
        .config
        .pick_count_dialog
        .clone())
}

#[tauri::command]
pub(crate) fn open_pick_count(
    app: AppHandle,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    create_pick_count_window(&app)?;
    if let Some(window) = app.get_webview_window("pick_count") {
        let _ = window.emit("pick-count-open", ());
        let _ = window.show();
        let _ = window.set_focus();
    }
    state
        .inner
        .lock()
        .map_err(|_| "状态锁定失败".to_string())?
        .floating_hidden_for_pick_count = true;
    hide_floating_window(&app);
    Ok(())
}

#[tauri::command]
pub(crate) fn cancel_pick_count(
    app: AppHandle,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("pick_count") {
        let _ = window.hide();
        let _ = window.emit("pick-count-stop-bgm", ());
    }
    state
        .inner
        .lock()
        .map_err(|_| "状态锁定失败".to_string())?
        .floating_hidden_for_pick_count = false;
    show_floating_window(&app);
    Ok(())
}

#[tauri::command]
pub(crate) fn confirm_pick_count(
    app: AppHandle,
    state: tauri::State<'_, AppState>,
    count: i32,
    play_music: bool,
) -> Result<(), String> {
    let selected_count = clamp_i32(count, 1, 10, 1);
    push_log(
        &app,
        &state,
        "info",
        &format!("Pick count confirmed. count={selected_count}, playMusic={play_music}"),
    );
    let picked_students = {
        let mut guard = state.inner.lock().map_err(|_| "状态锁定失败".to_string())?;
        if guard.config.allow_repeat_draw {
            if guard.weighted_pool_cache.is_none() {
                guard.weighted_pool_cache = Some(build_weighted_pool(&guard.config));
            }
            pick_students_with_repeat(guard.weighted_pool_cache.as_ref().unwrap(), selected_count)
        } else {
            pick_students_without_repeat(&guard.config, selected_count)
        }
    };
    if !picked_students.is_empty() {
        let names = picked_students
            .iter()
            .map(|student| student.name.as_str())
            .collect::<Vec<_>>()
            .join(", ");
        push_log(&app, &state, "info", &format!("Picked students: {names}"));
    }

    if let Some(window) = app.get_webview_window("pick_count") {
        let _ = window.hide();
    }

    {
        let mut guard = state.inner.lock().map_err(|_| "状态锁定失败".to_string())?;
        guard.floating_hidden_for_pick_count = true;
        guard.current_pick_results = picked_students.clone();
        guard.pick_result_token = guard.pick_result_token.saturating_add(1);
    }

    open_pick_result_window(&app, &state, picked_students)
}

#[tauri::command]
pub(crate) fn play_click_sound(state: tauri::State<'_, AppState>) -> Result<(), String> {
    state.audio.send(AudioCommand::PlayClick)
}

#[tauri::command]
pub(crate) fn play_bgm(state: tauri::State<'_, AppState>) -> Result<(), String> {
    state.audio.send(AudioCommand::PlayBgm)
}

#[tauri::command]
pub(crate) fn stop_bgm(state: tauri::State<'_, AppState>) -> Result<(), String> {
    state.audio.send(AudioCommand::StopBgm)
}

#[tauri::command]
pub(crate) fn play_gacha_sound(
    state: tauri::State<'_, AppState>,
    volume: f64,
) -> Result<(), String> {
    state.audio.send(AudioCommand::PlayGacha(volume as f32))
}

#[tauri::command]
pub(crate) fn stop_gacha_sound(state: tauri::State<'_, AppState>) -> Result<(), String> {
    state.audio.send(AudioCommand::StopGacha)
}
#[tauri::command]
pub(crate) fn get_pick_result_config(
    state: tauri::State<'_, AppState>,
) -> Result<PickResultDialogConfig, String> {
    Ok(state
        .inner
        .lock()
        .map_err(|_| "状态锁定失败".to_string())?
        .config
        .pick_result_dialog
        .clone())
}

#[tauri::command]
pub(crate) fn get_pick_results(
    state: tauri::State<'_, AppState>,
) -> Result<Vec<PickedStudent>, String> {
    Ok(state
        .inner
        .lock()
        .map_err(|_| "状态锁定失败".to_string())?
        .current_pick_results
        .clone())
}

#[tauri::command]
pub(crate) fn close_pick_result(
    app: AppHandle,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    let token = {
        let mut guard = state.inner.lock().map_err(|_| "状态锁定失败".to_string())?;
        guard.pick_result_token = guard.pick_result_token.saturating_add(1);
        guard.current_pick_results.clear();
        guard.floating_hidden_for_pick_count = false;
        guard.pick_result_token
    };
    if let Some(window) = app.get_webview_window("pick_result") {
        let _ = window.emit(
            "pick-result-reset",
            PickResultResetPayload {
                token,
                reason: "close".to_string(),
            },
        );
        let _ = window.hide();
    }
    if let Some(window) = app.get_webview_window("pick_count") {
        let _ = window.emit("pick-count-stop-bgm", ());
    }
    show_floating_window(&app);
    Ok(())
}

#[tauri::command]
pub(crate) fn get_config(state: tauri::State<'_, AppState>) -> Result<AppConfig, String> {
    cached_config(&state)
}

#[tauri::command]
pub(crate) fn parse_student_list_text(
    raw_text: String,
    existing_students: Vec<Student>,
) -> Result<StudentListParseResult, String> {
    Ok(parse_student_list_text_impl(&raw_text, &existing_students))
}

#[tauri::command]
pub(crate) fn import_student_list_from_file(
    existing_students: Vec<Student>,
) -> Result<Option<StudentListParseResult>, String> {
    let Some(path) = rfd::FileDialog::new()
        .add_filter("名单文件", &["txt", "csv"])
        .pick_file()
    else {
        return Ok(None);
    };
    let raw_text =
        fs::read_to_string(&path).map_err(|error| format!("读取名单文件失败: {error}"))?;
    Ok(Some(parse_student_list_text_impl(
        &raw_text,
        &existing_students,
    )))
}

#[tauri::command]
pub(crate) fn save_app_config(
    app: AppHandle,
    state: tauri::State<'_, AppState>,
    config: AppConfig,
) -> Result<ApiResult, String> {
    let normalized = normalize_config(config);
    save_config(&normalized)?;
    {
        let mut guard = state.inner.lock().map_err(|_| "状态锁定失败".to_string())?;
        guard.config = normalized.clone();
        guard.weighted_pool_cache = None;
    }
    if let Some(window) = app.get_webview_window("floating") {
        apply_floating_window_config(&window, &normalized);
    } else {
        create_floating_window(&app, &normalized)?;
    }
    push_log(&app, &state, "info", "配置保存成功，悬浮窗已自动刷新配置");
    Ok(ApiResult {
        ok: true,
        message: "配置保存成功，悬浮窗已自动刷新配置".to_string(),
        detail: None,
        restart_required: Some(false),
    })
}

#[tauri::command]
pub(crate) fn get_app_info(app: AppHandle) -> Result<AppInfo, String> {
    Ok(AppInfo {
        version: app.package_info().version.to_string(),
        is_debug_mode: cfg!(debug_assertions),
        is_admin: is_process_elevated(),
        exe_path: std::env::current_exe()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string(),
    })
}

#[tauri::command]
pub(crate) async fn check_update(app: AppHandle) -> Result<UpdateResult, String> {
    Ok(check_update_from_main(&app.package_info().version.to_string()).await)
}

#[tauri::command]
pub(crate) fn request_admin_elevation(
    app: AppHandle,
    state: tauri::State<'_, AppState>,
) -> Result<ApiResult, String> {
    if is_process_elevated() {
        return Ok(ApiResult {
            ok: true,
            message: "已在管理员权限下运行。".to_string(),
            detail: None,
            restart_required: None,
        });
    }
    let result = request_admin_relaunch();
    if result.ok {
        state
            .inner
            .lock()
            .map_err(|_| "状态锁定失败".to_string())?
            .is_quitting = true;
        let app_clone = app.clone();
        std::thread::spawn(move || {
            std::thread::sleep(std::time::Duration::from_millis(150));
            app_clone.exit(0);
        });
    }
    Ok(result)
}

#[tauri::command]
pub(crate) fn create_admin_startup_task(
    state: tauri::State<'_, AppState>,
    exe_path: String,
    task_name: String,
) -> Result<ApiResult, String> {
    let result = create_admin_startup_task_impl(&task_name, &exe_path);
    if result.ok {
        let mut config = cached_config(&state)?;
        config.web_config.admin_auto_start_enabled = true;
        config.web_config.admin_auto_start_path = exe_path.trim().to_string();
        config.web_config.admin_auto_start_task_name = if task_name.trim().is_empty() {
            ADMIN_TASK_DEFAULT_NAME.to_string()
        } else {
            task_name.trim().to_string()
        };
        save_config(&config)?;
        let mut guard = state.inner.lock().map_err(|_| "状态锁定失败".to_string())?;
        guard.config = config;
        guard.weighted_pool_cache = None;
    }
    Ok(result)
}

#[tauri::command]
pub(crate) fn renderer_log(
    app: AppHandle,
    state: tauri::State<'_, AppState>,
    level: String,
    text: String,
) {
    let level = if level.trim().is_empty() {
        "info"
    } else {
        level.trim()
    };
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    level.hash(&mut hasher);
    text.hash(&mut hasher);
    let key = hasher.finish();
    let now = std::time::Instant::now();
    if let Ok(mut guard) = state.inner.lock() {
        if let Some(last) = guard.log_dedup.get(&key) {
            if now.duration_since(*last).as_millis() < 1000 {
                return;
            }
        }
        guard.log_dedup.insert(key, now);
        if guard.log_dedup.len() > 100 {
            let cutoff = now - std::time::Duration::from_secs(10);
            guard.log_dedup.retain(|_, time| *time > cutoff);
        }
    }
    push_log(&app, &state, level, &text);
}

#[tauri::command]
pub(crate) fn get_logs(state: tauri::State<'_, AppState>) -> Result<Vec<LogEntry>, String> {
    Ok(state
        .inner
        .lock()
        .map_err(|_| "状态锁定失败".to_string())?
        .logs
        .iter()
        .cloned()
        .collect())
}
