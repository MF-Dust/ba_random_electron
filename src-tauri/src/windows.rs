use tauri::{
    AppHandle, Emitter, Manager, PhysicalPosition, PhysicalSize, Position, Size, WebviewUrl,
    WebviewWindow, WebviewWindowBuilder,
};

use crate::config::{save_config, AppConfig};
use crate::models::{PickResultOpenPayload, PickResultResetPayload, PickedStudent};
use crate::state::AppState;
fn route_url(route: &str) -> WebviewUrl {
    if route.is_empty() {
        WebviewUrl::App("index.html".into())
    } else {
        WebviewUrl::App(format!("index.html#{route}").into())
    }
}

fn floating_window_size(config: &AppConfig) -> (u32, u32) {
    let size_px = (50.0 * (config.floating_button.size_percent / 100.0)).round() as u32;
    let window_size = size_px.saturating_add(20).max(72);
    (window_size, window_size)
}

pub(crate) fn apply_floating_window_config(window: &WebviewWindow, config: &AppConfig) {
    let (width, height) = floating_window_size(config);
    let _ = window.set_size(Size::Physical(PhysicalSize { width, height }));
    let _ = window.set_always_on_top(config.floating_button.always_on_top);
    if let (Some(x), Some(y)) = (
        config.floating_button.position.x,
        config.floating_button.position.y,
    ) {
        let _ = window.set_position(Position::Physical(PhysicalPosition { x, y }));
    }
    let _ = window.emit("floating-config-updated", &config.floating_button);
}

pub(crate) fn create_floating_window(
    app: &AppHandle,
    config: &AppConfig,
) -> Result<WebviewWindow, String> {
    if let Some(window) = app.get_webview_window("floating") {
        apply_floating_window_config(&window, config);
        let _ = window.show();
        return Ok(window);
    }

    let (width, height) = floating_window_size(config);
    let mut builder = WebviewWindowBuilder::new(app, "floating", route_url(""))
        .title("KVRandom")
        .inner_size(width as f64, height as f64)
        .decorations(false)
        .resizable(false)
        .maximizable(false)
        .minimizable(false)
        .transparent(true)
        .shadow(false)
        .skip_taskbar(true)
        .always_on_top(config.floating_button.always_on_top);

    if let (Some(x), Some(y)) = (
        config.floating_button.position.x,
        config.floating_button.position.y,
    ) {
        builder = builder.position(x as f64, y as f64);
    }

    builder
        .build()
        .map_err(|error| format!("创建悬浮窗失败: {error}"))
}

pub(crate) fn create_pick_count_window(app: &AppHandle) -> Result<WebviewWindow, String> {
    if let Some(window) = app.get_webview_window("pick_count") {
        return Ok(window);
    }

    WebviewWindowBuilder::new(app, "pick_count", route_url("/pick-count"))
        .title("选择人数")
        .decorations(false)
        .resizable(false)
        .maximizable(false)
        .minimizable(false)
        .transparent(true)
        .fullscreen(true)
        .visible(false)
        .skip_taskbar(!cfg!(debug_assertions))
        .always_on_top(true)
        .build()
        .map_err(|error| format!("创建人数选择窗口失败: {error}"))
}

pub(crate) fn create_pick_result_window(app: &AppHandle) -> Result<WebviewWindow, String> {
    if let Some(window) = app.get_webview_window("pick_result") {
        return Ok(window);
    }

    WebviewWindowBuilder::new(app, "pick_result", route_url("/pick-result"))
        .title("抽取结果")
        .decorations(false)
        .resizable(false)
        .maximizable(false)
        .minimizable(false)
        .transparent(true)
        .fullscreen(true)
        .visible(false)
        .skip_taskbar(!cfg!(debug_assertions))
        .always_on_top(true)
        .build()
        .map_err(|error| format!("创建结果窗口失败: {error}"))
}

pub(crate) fn open_config_window(app: &AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("config") {
        let _ = window.show();
        let _ = window.set_focus();
        return Ok(());
    }

    WebviewWindowBuilder::new(app, "config", route_url("/config"))
        .title("KVRandom 配置")
        .inner_size(1120.0, 760.0)
        .resizable(true)
        .build()
        .map(|window| {
            let _ = window.set_focus();
        })
        .map_err(|error| format!("创建配置窗口失败: {error}"))
}

pub(crate) fn hide_floating_window(app: &AppHandle) {
    if let Some(window) = app.get_webview_window("floating") {
        let _ = window.hide();
    }
}

pub(crate) fn show_floating_window(app: &AppHandle) {
    if let Some(window) = app.get_webview_window("floating") {
        let _ = window.show();
        let _ = window.set_focus();
    }
}

pub(crate) fn persist_floating_position(app: &AppHandle, state: &tauri::State<'_, AppState>) {
    let Some(window) = app.get_webview_window("floating") else {
        return;
    };
    let Ok(position) = window.outer_position() else {
        return;
    };
    let Ok(mut guard) = state.inner.lock() else {
        return;
    };
    guard.config.floating_button.position.x = Some(position.x);
    guard.config.floating_button.position.y = Some(position.y);
    let config_ref = &guard.config;
    let _ = save_config(config_ref);
}

pub(crate) fn open_pick_result_window(
    app: &AppHandle,
    state: &tauri::State<'_, AppState>,
    results: Vec<PickedStudent>,
) -> Result<(), String> {
    create_pick_result_window(app)?;
    let token = {
        let guard = state.inner.lock().map_err(|_| "状态锁定失败".to_string())?;
        guard.pick_result_token
    };

    if let Some(window) = app.get_webview_window("pick_result") {
        let _ = window.emit(
            "pick-result-reset",
            PickResultResetPayload {
                token,
                reason: "before-open".to_string(),
            },
        );
        let _ = window.emit("pick-result-open", PickResultOpenPayload { token, results });
        let _ = window.show();
        let _ = window.set_focus();
    }
    hide_floating_window(app);
    Ok(())
}
