mod admin;
mod audio;
mod commands;
mod config;
mod models;
mod picker;
mod state;
mod tray;
mod update;
mod utils;
mod windows;

use std::sync::Mutex;

use admin::{acquire_single_instance_guard, is_process_elevated, request_admin_relaunch};
use audio::AudioController;
use config::load_config;
use state::{AppState, RuntimeState};
use tauri::Manager;
use tray::setup_tray;
use windows::{
    create_floating_window, create_pick_count_window, create_pick_result_window,
    persist_floating_position,
};

pub fn run() {
    let single_instance_guard = match acquire_single_instance_guard() {
        Ok(Some(guard)) => guard,
        Ok(None) => return,
        Err(error) => {
            eprintln!("{error}");
            return;
        }
    };

    tauri::Builder::default()
        .setup(move |app| {
            let app_handle = app.handle().clone();
            let initial_config = load_config(&app_handle).unwrap_or_default();
            let mut single_instance_guard = Some(single_instance_guard);

            if initial_config.web_config.admin_topmost_enabled
                && cfg!(target_os = "windows")
                && !cfg!(debug_assertions)
                && !is_process_elevated()
            {
                drop(single_instance_guard.take());
                let result = request_admin_relaunch();
                if result.ok {
                    app_handle.exit(0);
                    return Ok(());
                }

                single_instance_guard = match acquire_single_instance_guard() {
                    Ok(Some(guard)) => Some(guard),
                    Ok(None) => {
                        app_handle.exit(0);
                        return Ok(());
                    }
                    Err(error) => return Err(anyhow::Error::msg(error).into()),
                };
            }

            let single_instance_guard = single_instance_guard
                .take()
                .ok_or_else(|| anyhow::Error::msg("单实例锁未初始化"))?;
            app.manage(AppState {
                inner: Mutex::new(RuntimeState::new(initial_config.clone())),
                audio: AudioController::new(&app_handle),
                _single_instance_guard: single_instance_guard,
            });

            setup_tray(&app_handle).map_err(anyhow::Error::msg)?;
            create_floating_window(&app_handle, &initial_config).map_err(anyhow::Error::msg)?;
            create_pick_count_window(&app_handle).map_err(anyhow::Error::msg)?;
            create_pick_result_window(&app_handle).map_err(anyhow::Error::msg)?;
            Ok(())
        })
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { .. } = event {
                if window.label() == "floating" {
                    let app = window.app_handle();
                    if let Some(state) = app.try_state::<AppState>() {
                        persist_floating_position(app, &state);
                    }
                }
            }
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_floating_button_config,
            commands::floating_button_clicked,
            commands::floating_button_drag_start,
            commands::floating_button_drag_move,
            commands::floating_button_drag_end,
            commands::floating_button_set_ignore_mouse,
            commands::get_pick_count_config,
            commands::open_pick_count,
            commands::cancel_pick_count,
            commands::confirm_pick_count,
            commands::play_click_sound,
            commands::play_bgm,
            commands::stop_bgm,
            commands::play_gacha_sound,
            commands::stop_gacha_sound,
            commands::get_pick_result_config,
            commands::get_pick_results,
            commands::close_pick_result,
            commands::get_config,
            commands::parse_student_list_text,
            commands::import_student_list_from_file,
            commands::save_app_config,
            commands::get_app_info,
            commands::check_update,
            commands::request_admin_elevation,
            commands::create_admin_startup_task,
            commands::renderer_log,
            commands::get_logs
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
