use std::fs;
use std::path::{Path, PathBuf};
use tauri::{AppHandle, Manager};

pub(crate) fn clamp_f64(value: f64, min: f64, max: f64, fallback: f64) -> f64 {
    if value.is_finite() {
        value.clamp(min, max)
    } else {
        fallback
    }
}

pub(crate) fn clamp_i32(value: i32, min: i32, max: i32, fallback: i32) -> i32 {
    if min <= max {
        value.clamp(min, max)
    } else {
        fallback
    }
}

fn asset_candidates_from_base(base: &Path, relative: &str) -> [PathBuf; 4] {
    [
        base.join("public").join(relative),
        base.join("_up_").join("public").join(relative),
        base.join(relative),
        base.join("_up_").join(relative),
    ]
}

pub(crate) fn load_asset_bytes(app: &AppHandle, relative_path: &str) -> Vec<u8> {
    let relative = relative_path.trim_start_matches('/');
    let mut candidates = Vec::new();
    if let Ok(current_dir) = std::env::current_dir() {
        candidates.extend(asset_candidates_from_base(&current_dir, relative));
        if current_dir
            .file_name()
            .and_then(|name| name.to_str())
            .is_some_and(|name| name == "src-tauri")
        {
            if let Some(project_dir) = current_dir.parent() {
                candidates.extend(asset_candidates_from_base(project_dir, relative));
            }
        }
    }
    if let Ok(resource_dir) = app.path().resource_dir() {
        candidates.extend(asset_candidates_from_base(&resource_dir, relative));
    }
    candidates
        .into_iter()
        .find_map(|path| fs::read(path).ok())
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn asset_candidates_include_tauri_packaged_resource_layout() {
        let base = Path::new("app");
        let candidates = asset_candidates_from_base(base, "sound/bgm.mp3");

        assert!(candidates.contains(&PathBuf::from("app/public/sound/bgm.mp3")));
        assert!(candidates.contains(&PathBuf::from("app/_up_/public/sound/bgm.mp3")));
        assert!(candidates.contains(&PathBuf::from("app/sound/bgm.mp3")));
        assert!(candidates.contains(&PathBuf::from("app/_up_/sound/bgm.mp3")));
    }
}
