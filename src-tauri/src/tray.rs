use tauri::menu::{Menu, MenuItem};
use tauri::tray::TrayIconBuilder;
use tauri::{AppHandle, Manager};

use crate::state::AppState;
use crate::windows::{open_config_window, persist_floating_position};
pub(crate) fn setup_tray(app: &AppHandle) -> Result<(), String> {
    let config_item = MenuItem::with_id(app, "open_config", "设置面板～", true, None::<&str>)
        .map_err(|e| e.to_string())?;
    let quit_item = MenuItem::with_id(app, "quit", "阿罗娜先走啦～", true, None::<&str>)
        .map_err(|e| e.to_string())?;
    let menu = Menu::with_items(app, &[&config_item, &quit_item]).map_err(|e| e.to_string())?;
    let mut builder = TrayIconBuilder::new()
        .tooltip("KVRandom - 阿罗娜在这里哦～")
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_menu_event(|app, event| match event.id().as_ref() {
            "open_config" => {
                let _ = open_config_window(app);
            }
            "quit" => {
                if let Some(state) = app.try_state::<AppState>() {
                    persist_floating_position(app, &state);
                    if let Ok(mut guard) = state.inner.lock() {
                        guard.is_quitting = true;
                    }
                }
                app.exit(0);
            }
            _ => {}
        });

    if let Some(icon) = app.default_window_icon() {
        builder = builder.icon(icon.clone());
    }
    builder.build(app).map_err(|error| error.to_string())?;
    Ok(())
}
