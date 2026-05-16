use std::fs;
use std::hash::{Hash, Hasher};
use tauri::{AppHandle, Manager, PhysicalPosition, Position, WebviewWindow};

use crate::admin::{create_admin_startup_task_impl, is_process_elevated, request_admin_relaunch};
use crate::audio::AudioCommand;
use crate::config::{
    current_config_signature, normalize_config_value, parse_student_list_text_impl, save_config,
    AppConfig, FloatingButtonConfig, PickCountDialogConfig, PickResultDialogConfig, Student,
    StudentListParseResult, ADMIN_TASK_DEFAULT_NAME, MAX_PICK_COUNT, MIN_PICK_COUNT,
};
use crate::models::{ApiResult, AppInfo, PickedStudent, UpdateResult};
use crate::picker::{build_weighted_pool, pick_students_with_repeat, pick_students_without_repeat};
use crate::state::{push_log, refresh_config, AppState, DragSession, LogEntry};
use crate::update::check_update_from_main;
use crate::utils::clamp_i32;
use crate::windows::{
    apply_floating_window_config, create_floating_window, create_pick_count_window,
    create_pick_result_window, hide_floating_window, hide_pick_count_window,
    open_pick_count_window, open_pick_result_window, persist_floating_position,
    reset_and_hide_pick_result_window, show_floating_window, stop_pick_count_bgm,
};

#[tauri::command]
pub(crate) async fn get_floating_button_config(
    app: AppHandle,
) -> Result<FloatingButtonConfig, String> {
    tauri::async_runtime::spawn_blocking(move || {
        let state = app.state::<AppState>();
        let config = refresh_config(&app, &state)?;
        if let Some(window) = app.get_webview_window("floating") {
            apply_floating_window_config(&window, &config);
        }
        Ok(config.floating_button)
    })
    .await
    .map_err(|e| e.to_string())?
}

#[tauri::command]
pub(crate) async fn floating_button_clicked(
    app: AppHandle,
) -> Result<(), String> {
    open_pick_count(app).await
}

#[tauri::command]
pub(crate) async fn floating_button_drag_start(
    window: WebviewWindow,
) -> Result<(), String> {
    tauri::async_runtime::spawn_blocking(move || {
        let state = window.state::<AppState>();
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
    })
    .await
    .map_err(|e| e.to_string())?
}

#[tauri::command]
pub(crate) async fn floating_button_drag_move(
    window: WebviewWindow,
    dx: f64,
    dy: f64,
) -> Result<(), String> {
    tauri::async_runtime::spawn_blocking(move || {
        let state = window.state::<AppState>();
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
    })
    .await
    .map_err(|e| e.to_string())?
}

#[tauri::command]
pub(crate) async fn floating_button_drag_end(
    app: AppHandle,
) -> Result<(), String> {
    tauri::async_runtime::spawn_blocking(move || {
        let state = app.state::<AppState>();
        state
            .inner
            .lock()
            .map_err(|_| "状态锁定失败".to_string())?
            .drag_session = None;
        persist_floating_position(&app, &state);
        Ok(())
    })
    .await
    .map_err(|e| e.to_string())?
}

#[tauri::command]
pub(crate) async fn floating_button_set_ignore_mouse(
    window: WebviewWindow,
    _ignore: bool,
) -> Result<(), String> {
    tauri::async_runtime::spawn_blocking(move || {
        window
            .set_ignore_cursor_events(false)
            .map_err(|error| error.to_string())
    })
    .await
    .map_err(|e| e.to_string())?
}

#[tauri::command]
pub(crate) async fn prewarm_aux_windows(app: AppHandle) -> Result<(), String> {
    tauri::async_runtime::spawn_blocking(move || {
        create_pick_count_window(&app)?;
        create_pick_result_window(&app)?;
        Ok(())
    })
    .await
    .map_err(|e| e.to_string())?
}

#[tauri::command]
pub(crate) async fn get_pick_count_config(
    app: AppHandle,
) -> Result<PickCountDialogConfig, String> {
    tauri::async_runtime::spawn_blocking(move || {
        let state = app.state::<AppState>();
        let config = refresh_config(&app, &state)?;
        Ok(config.pick_count_dialog)
    })
    .await
    .map_err(|e| e.to_string())?
}

#[tauri::command]
pub(crate) async fn open_pick_count(
    app: AppHandle,
) -> Result<(), String> {
    tauri::async_runtime::spawn_blocking(move || {
        let state = app.state::<AppState>();
        let config = refresh_config(&app, &state)?;
        if let Some(window) = app.get_webview_window("floating") {
            apply_floating_window_config(&window, &config);
        }
        open_pick_count_window(&app, &config.pick_count_dialog)?;
        state
            .inner
            .lock()
            .map_err(|_| "状态锁定失败".to_string())?
            .floating_hidden_for_pick_count = true;
        hide_floating_window(&app);
        Ok(())
    })
    .await
    .map_err(|e| e.to_string())?
}

#[tauri::command]
pub(crate) async fn cancel_pick_count(
    app: AppHandle,
) -> Result<(), String> {
    tauri::async_runtime::spawn_blocking(move || {
        let state = app.state::<AppState>();
        hide_pick_count_window(&app);
        stop_pick_count_bgm(&app);
        state
            .inner
            .lock()
            .map_err(|_| "状态锁定失败".to_string())?
            .floating_hidden_for_pick_count = false;
        show_floating_window(&app);
        Ok(())
    })
    .await
    .map_err(|e| e.to_string())?
}

#[tauri::command]
pub(crate) async fn confirm_pick_count(
    app: AppHandle,
    count: i32,
    play_music: bool,
) -> Result<(), String> {
    tauri::async_runtime::spawn_blocking(move || {
        let state = app.state::<AppState>();
        let selected_count = clamp_i32(count, MIN_PICK_COUNT, MAX_PICK_COUNT, MIN_PICK_COUNT);
        let config = refresh_config(&app, &state)?;
        push_log(
            &app,
            &state,
            "info",
            &format!("Pick count confirmed. count={selected_count}, playMusic={play_music}"),
        );
        if let Some(window) = app.get_webview_window("floating") {
            apply_floating_window_config(&window, &config);
        }
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

        hide_pick_count_window(&app);

        {
            let mut guard = state.inner.lock().map_err(|_| "状态锁定失败".to_string())?;
            guard.floating_hidden_for_pick_count = true;
            guard.current_pick_results = picked_students.clone();
            guard.pick_result_token = guard.pick_result_token.saturating_add(1);
        }

        open_pick_result_window(&app, &state, picked_students)
    })
    .await
    .map_err(|e| e.to_string())?
}

#[tauri::command]
pub(crate) async fn play_click_sound(app: AppHandle) -> Result<(), String> {
    tauri::async_runtime::spawn_blocking(move || {
        let state = app.state::<AppState>();
        state.audio.send(AudioCommand::PlayClick)
    })
    .await
    .map_err(|e| e.to_string())?
}

#[tauri::command]
pub(crate) async fn play_bgm(app: AppHandle) -> Result<(), String> {
    tauri::async_runtime::spawn_blocking(move || {
        let state = app.state::<AppState>();
        state.audio.send(AudioCommand::PlayBgm)
    })
    .await
    .map_err(|e| e.to_string())?
}

#[tauri::command]
pub(crate) async fn stop_bgm(app: AppHandle) -> Result<(), String> {
    tauri::async_runtime::spawn_blocking(move || {
        let state = app.state::<AppState>();
        state.audio.send(AudioCommand::StopBgm)
    })
    .await
    .map_err(|e| e.to_string())?
}

#[tauri::command]
pub(crate) async fn play_gacha_sound(
    app: AppHandle,
    volume: f64,
) -> Result<(), String> {
    tauri::async_runtime::spawn_blocking(move || {
        let state = app.state::<AppState>();
        state.audio.send(AudioCommand::PlayGacha(volume as f32))
    })
    .await
    .map_err(|e| e.to_string())?
}

#[tauri::command]
pub(crate) async fn stop_gacha_sound(app: AppHandle) -> Result<(), String> {
    tauri::async_runtime::spawn_blocking(move || {
        let state = app.state::<AppState>();
        state.audio.send(AudioCommand::StopGacha)
    })
    .await
    .map_err(|e| e.to_string())?
}

#[tauri::command]
pub(crate) async fn get_pick_result_config(
    app: AppHandle,
) -> Result<PickResultDialogConfig, String> {
    tauri::async_runtime::spawn_blocking(move || {
        let state = app.state::<AppState>();
        let config = refresh_config(&app, &state)?;
        Ok(config.pick_result_dialog)
    })
    .await
    .map_err(|e| e.to_string())?
}

#[tauri::command]
pub(crate) async fn get_pick_results(
    app: AppHandle,
) -> Result<Vec<PickedStudent>, String> {
    tauri::async_runtime::spawn_blocking(move || {
        let state = app.state::<AppState>();
        let results = state
            .inner
            .lock()
            .map_err(|_| "状态锁定失败".to_string())?
            .current_pick_results
            .clone();
        Ok(results)
    })
    .await
    .map_err(|e| e.to_string())?
}

#[tauri::command]
pub(crate) async fn close_pick_result(
    app: AppHandle,
) -> Result<(), String> {
    tauri::async_runtime::spawn_blocking(move || {
        let state = app.state::<AppState>();
        let token = {
            let mut guard = state.inner.lock().map_err(|_| "状态锁定失败".to_string())?;
            guard.pick_result_token = guard.pick_result_token.saturating_add(1);
            guard.current_pick_results.clear();
            guard.floating_hidden_for_pick_count = false;
            guard.pick_result_token
        };
        reset_and_hide_pick_result_window(&app, token, "close");
        stop_pick_count_bgm(&app);
        show_floating_window(&app);
        Ok(())
    })
    .await
    .map_err(|e| e.to_string())?
}

#[tauri::command]
pub(crate) async fn get_config(
    app: AppHandle,
) -> Result<AppConfig, String> {
    tauri::async_runtime::spawn_blocking(move || {
        let state = app.state::<AppState>();
        let config = refresh_config(&app, &state)?;
        if let Some(window) = app.get_webview_window("floating") {
            apply_floating_window_config(&window, &config);
        }
        Ok(config)
    })
    .await
    .map_err(|e| e.to_string())?
}

#[tauri::command]
pub(crate) async fn parse_student_list_text(
    raw_text: String,
    existing_students: Vec<Student>,
) -> Result<StudentListParseResult, String> {
    tauri::async_runtime::spawn_blocking(move || {
        Ok(parse_student_list_text_impl(&raw_text, &existing_students))
    })
    .await
    .map_err(|e| e.to_string())?
}

#[tauri::command]
pub(crate) async fn import_student_list_from_file(
    existing_students: Vec<Student>,
) -> Result<Option<StudentListParseResult>, String> {
    tauri::async_runtime::spawn_blocking(move || {
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
    })
    .await
    .map_err(|e| e.to_string())?
}

#[tauri::command]
pub(crate) async fn save_app_config(
    app: AppHandle,
    config: serde_json::Value,
) -> Result<ApiResult, String> {
    tauri::async_runtime::spawn_blocking(move || {
        let state = app.state::<AppState>();
        let normalized = normalize_config_value(config);
        save_config(&normalized)?;
        let config_signature = current_config_signature().ok().flatten();
        {
            let mut guard = state.inner.lock().map_err(|_| "状态锁定失败".to_string())?;
            guard.apply_config(normalized.clone(), config_signature, true);
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
    })
    .await
    .map_err(|e| e.to_string())?
}

#[tauri::command]
pub(crate) async fn get_app_info(app: AppHandle) -> Result<AppInfo, String> {
    tauri::async_runtime::spawn_blocking(move || {
        Ok(AppInfo {
            version: app.package_info().version.to_string(),
            is_debug_mode: cfg!(debug_assertions),
            is_admin: is_process_elevated(),
            exe_path: std::env::current_exe()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string(),
        })
    })
    .await
    .map_err(|e| e.to_string())?
}

#[tauri::command]
pub(crate) async fn check_update(app: AppHandle) -> Result<UpdateResult, String> {
    Ok(check_update_from_main(&app.package_info().version.to_string()).await)
}

#[tauri::command]
pub(crate) async fn request_admin_elevation(
    app: AppHandle,
) -> Result<ApiResult, String> {
    tauri::async_runtime::spawn_blocking(move || {
        let state = app.state::<AppState>();
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
    })
    .await
    .map_err(|e| e.to_string())?
}

#[tauri::command]
pub(crate) async fn create_admin_startup_task(
    app: AppHandle,
    exe_path: String,
    task_name: String,
) -> Result<ApiResult, String> {
    tauri::async_runtime::spawn_blocking(move || {
        let state = app.state::<AppState>();
        let result = create_admin_startup_task_impl(&task_name, &exe_path);
        if result.ok {
            let mut config = refresh_config(&app, &state)?;
            config.web_config.admin_auto_start_enabled = true;
            config.web_config.admin_auto_start_path = exe_path.trim().to_string();
            config.web_config.admin_auto_start_task_name = if task_name.trim().is_empty() {
                ADMIN_TASK_DEFAULT_NAME.to_string()
            } else {
                task_name.trim().to_string()
            };
            save_config(&config)?;
            let config_signature = current_config_signature().ok().flatten();
            let mut guard = state.inner.lock().map_err(|_| "状态锁定失败".to_string())?;
            guard.apply_config(config, config_signature, true);
        }
        Ok(result)
    })
    .await
    .map_err(|e| e.to_string())?
}

#[tauri::command]
pub(crate) async fn renderer_log(
    app: AppHandle,
    level: String,
    text: String,
) -> Result<(), String> {
    tauri::async_runtime::spawn_blocking(move || {
        let state = app.state::<AppState>();
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
                    return Ok(());
                }
            }
            guard.log_dedup.insert(key, now);
            if guard.log_dedup.len() > 100 {
                let cutoff = now - std::time::Duration::from_secs(10);
                guard.log_dedup.retain(|_, time| *time > cutoff);
            }
        }
        push_log(&app, &state, level, &text);
        Ok(())
    })
    .await
    .map_err(|e| e.to_string())?
}

#[tauri::command]
pub(crate) async fn get_logs(app: AppHandle) -> Result<Vec<LogEntry>, String> {
    tauri::async_runtime::spawn_blocking(move || {
        let state = app.state::<AppState>();
        let logs = state
            .inner
            .lock()
            .map_err(|_| "状态锁定失败".to_string())?
            .logs
            .iter()
            .cloned()
            .collect();
        Ok(logs)
    })
    .await
    .map_err(|e| e.to_string())?
}
