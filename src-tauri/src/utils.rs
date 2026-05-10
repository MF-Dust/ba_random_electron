use std::fs;
use tauri::{AppHandle, Manager};

pub(crate) fn clamp_f64(value: f64, min: f64, max: f64, fallback: f64) -> f64 {
    if value.is_finite() {
        value.clamp(min, max)
    } else {
        fallback
    }
}

pub(crate) fn clamp_i32(value: i32, min: i32, max: i32, fallback: i32) -> i32 {
    if min <= value && value <= max {
        value
    } else {
        fallback.clamp(min, max)
    }
}
pub(crate) fn load_asset_bytes(app: &AppHandle, relative_path: &str) -> Vec<u8> {
    let relative = relative_path.trim_start_matches('/');
    let mut candidates = Vec::new();
    if let Ok(current_dir) = std::env::current_dir() {
        candidates.push(current_dir.join("public").join(relative));
        candidates.push(current_dir.join(relative));
        if current_dir
            .file_name()
            .and_then(|name| name.to_str())
            .is_some_and(|name| name == "src-tauri")
        {
            if let Some(project_dir) = current_dir.parent() {
                candidates.push(project_dir.join("public").join(relative));
            }
        }
    }
    if let Ok(resource_dir) = app.path().resource_dir() {
        candidates.push(resource_dir.join("public").join(relative));
        candidates.push(resource_dir.join(relative));
    }
    candidates
        .into_iter()
        .find_map(|path| fs::read(path).ok())
        .unwrap_or_default()
}
