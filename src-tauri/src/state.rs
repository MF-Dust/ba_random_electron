use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::sync::Mutex;
use tauri::{AppHandle, Emitter};

use crate::admin::SingleInstanceGuard;
use crate::audio::AudioController;
use crate::config::{load_config, AppConfig};
use crate::models::PickedStudent;
use crate::picker::WeightedPool;

const LOG_BUFFER_LIMIT: usize = 600;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct LogEntry {
    pub(crate) id: String,
    pub(crate) level: String,
    pub(crate) text: String,
    pub(crate) time: String,
}

#[derive(Debug, Clone)]
pub(crate) struct DragSession {
    pub(crate) start_x: i32,
    pub(crate) start_y: i32,
    pub(crate) last_x: i32,
    pub(crate) last_y: i32,
}

pub(crate) struct RuntimeState {
    pub(crate) config: AppConfig,
    pub(crate) weighted_pool_cache: Option<WeightedPool>,
    pub(crate) current_pick_results: Vec<PickedStudent>,
    pub(crate) pick_result_token: u64,
    pub(crate) drag_session: Option<DragSession>,
    pub(crate) floating_hidden_for_pick_count: bool,
    pub(crate) is_quitting: bool,
    pub(crate) logs: VecDeque<LogEntry>,
    pub(crate) log_dedup: HashMap<u64, std::time::Instant>,
}

impl RuntimeState {
    pub(crate) fn new(config: AppConfig) -> Self {
        Self {
            config,
            weighted_pool_cache: None,
            current_pick_results: Vec::new(),
            pick_result_token: 0,
            drag_session: None,
            floating_hidden_for_pick_count: false,
            is_quitting: false,
            logs: VecDeque::new(),
            log_dedup: HashMap::new(),
        }
    }
}

pub(crate) struct AppState {
    pub(crate) inner: Mutex<RuntimeState>,
    pub(crate) audio: AudioController,
    pub(crate) _single_instance_guard: SingleInstanceGuard,
}

pub(crate) fn refresh_config(
    app: &AppHandle,
    state: &tauri::State<'_, AppState>,
) -> Result<AppConfig, String> {
    let config = load_config(app)?;
    let mut guard = state.inner.lock().map_err(|_| "状态锁定失败".to_string())?;
    guard.config = config.clone();
    guard.weighted_pool_cache = None;
    Ok(config)
}

pub(crate) fn push_log(
    app: &AppHandle,
    state: &tauri::State<'_, AppState>,
    level: &str,
    text: &str,
) {
    let now = Utc::now();
    let entry = LogEntry {
        id: format!("{}-{}", now.timestamp_millis(), rand::random::<u64>()),
        level: level.to_string(),
        text: text.to_string(),
        time: now.to_rfc3339(),
    };
    if let Ok(mut guard) = state.inner.lock() {
        guard.logs.push_back(entry.clone());
        while guard.logs.len() > LOG_BUFFER_LIMIT {
            guard.logs.pop_front();
        }
    }
    let _ = app.emit("log-entry", entry);
}
