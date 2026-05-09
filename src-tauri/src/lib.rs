use chrono::Utc;
use rand::seq::SliceRandom;
use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::VecDeque;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::{Mutex, OnceLock};
use std::time::Duration;
use tauri::menu::{Menu, MenuItem};
use tauri::tray::TrayIconBuilder;
use tauri::{
    AppHandle, Emitter, Manager, PhysicalPosition, PhysicalSize, Position, Size, WebviewUrl,
    WebviewWindow, WebviewWindowBuilder,
};

const ADMIN_TASK_DEFAULT_NAME: &str = "Blue Random (Admin)";
const LOG_BUFFER_LIMIT: usize = 600;
const WEIGHT_BOOST_GAMMA: f64 = 1.5;
static HTTP_CLIENT: OnceLock<reqwest::Client> = OnceLock::new();

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Student {
    name: String,
    weight: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct FloatingPosition {
    x: Option<i32>,
    y: Option<i32>,
}

impl Default for FloatingPosition {
    fn default() -> Self {
        Self { x: None, y: None }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct FloatingButtonConfig {
    size_percent: f64,
    transparency_percent: f64,
    always_on_top: bool,
    position: FloatingPosition,
}

impl Default for FloatingButtonConfig {
    fn default() -> Self {
        Self {
            size_percent: 100.0,
            transparency_percent: 20.0,
            always_on_top: true,
            position: FloatingPosition::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PickCountDialogConfig {
    default_play_music: bool,
    background_darkness_percent: f64,
    default_count: i32,
}

impl Default for PickCountDialogConfig {
    fn default() -> Self {
        Self {
            default_play_music: false,
            background_darkness_percent: 50.0,
            default_count: 1,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PickResultDialogConfig {
    default_play_gacha_sound: bool,
    gacha_sound_volume: f64,
}

impl Default for PickResultDialogConfig {
    fn default() -> Self {
        Self {
            default_play_gacha_sound: true,
            gacha_sound_volume: 0.6,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct WebConfig {
    port: i32,
    admin_topmost_enabled: bool,
    admin_auto_start_enabled: bool,
    admin_auto_start_path: String,
    admin_auto_start_task_name: String,
}

impl Default for WebConfig {
    fn default() -> Self {
        Self {
            port: 21219,
            admin_topmost_enabled: false,
            admin_auto_start_enabled: false,
            admin_auto_start_path: String::new(),
            admin_auto_start_task_name: ADMIN_TASK_DEFAULT_NAME.to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AppConfig {
    student_list: Vec<Student>,
    allow_repeat_draw: bool,
    floating_button: FloatingButtonConfig,
    pick_count_dialog: PickCountDialogConfig,
    pick_result_dialog: PickResultDialogConfig,
    web_config: WebConfig,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            student_list: Vec::new(),
            allow_repeat_draw: true,
            floating_button: FloatingButtonConfig::default(),
            pick_count_dialog: PickCountDialogConfig::default(),
            pick_result_dialog: PickResultDialogConfig::default(),
            web_config: WebConfig::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PickedStudent {
    name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct LogEntry {
    id: String,
    level: String,
    text: String,
    time: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct ApiResult {
    ok: bool,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    detail: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    restart_required: Option<bool>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct AppInfo {
    version: String,
    is_debug_mode: bool,
    is_admin: bool,
    exe_path: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct UpdateResult {
    ok: bool,
    status: String,
    title: String,
    detail: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    commit_url: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    release_url: String,
    local_version: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    remote_version: String,
    debug: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct PickResultOpenPayload {
    token: u64,
    results: Vec<PickedStudent>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct PickResultResetPayload {
    token: u64,
    reason: String,
}

#[derive(Debug, Clone)]
struct DragSession {
    start_x: i32,
    start_y: i32,
    last_x: i32,
    last_y: i32,
}

struct RuntimeState {
    config: AppConfig,
    current_pick_results: Vec<PickedStudent>,
    pick_result_token: u64,
    drag_session: Option<DragSession>,
    floating_hidden_for_pick_count: bool,
    is_quitting: bool,
    logs: VecDeque<LogEntry>,
}

impl RuntimeState {
    fn new(config: AppConfig) -> Self {
        Self {
            config,
            current_pick_results: Vec::new(),
            pick_result_token: 0,
            drag_session: None,
            floating_hidden_for_pick_count: false,
            is_quitting: false,
            logs: VecDeque::new(),
        }
    }
}

struct AppState {
    inner: Mutex<RuntimeState>,
}

fn clamp_f64(value: f64, min: f64, max: f64, fallback: f64) -> f64 {
    if value.is_finite() {
        value.clamp(min, max)
    } else {
        fallback
    }
}

fn clamp_i32(value: i32, min: i32, max: i32, fallback: i32) -> i32 {
    if min <= value && value <= max {
        value
    } else {
        fallback.clamp(min, max)
    }
}

fn config_path() -> Result<PathBuf, String> {
    std::env::current_dir()
        .map(|dir| dir.join("config.yml"))
        .map_err(|error| format!("获取当前目录失败: {error}"))
}

fn legacy_config_path(app: &AppHandle) -> Option<PathBuf> {
    app.path()
        .app_data_dir()
        .ok()
        .map(|dir| dir.join("config.yml"))
}

fn escape_yaml_string(value: &str) -> String {
    value.replace('\\', "\\\\").replace('"', "\\\"")
}

fn to_config_yaml_with_comments(config: &AppConfig) -> String {
    let fb = &config.floating_button;
    let pick = &config.pick_count_dialog;
    let pick_result = &config.pick_result_dialog;
    let web = &config.web_config;
    let pos_x = fb
        .position
        .x
        .map(|value| value.to_string())
        .unwrap_or_else(|| "null".to_string());
    let pos_y = fb
        .position
        .y
        .map(|value| value.to_string())
        .unwrap_or_else(|| "null".to_string());
    let student_lines = if config.student_list.is_empty() {
        " []".to_string()
    } else {
        let lines = config
            .student_list
            .iter()
            .map(|student| {
                format!(
                    "  - name: \"{}\"\n    weight: {}",
                    escape_yaml_string(&student.name),
                    student.weight
                )
            })
            .collect::<Vec<_>>()
            .join("\n");
        format!("\n{lines}")
    };

    [
        "# 抽取名单列表".to_string(),
        format!("studentList:{student_lines}"),
        format!("allowRepeatDraw: {}", config.allow_repeat_draw),
        String::new(),
        "# 悬浮按钮配置".to_string(),
        "floatingButton:".to_string(),
        "  # 按钮大小百分比（基准 50px*50px），范围 0-1000，默认 100".to_string(),
        format!("  sizePercent: {}", fb.size_percent),
        "  # 透明度百分比，范围 0-100（0=完全不透明，100=完全透明），默认 20".to_string(),
        format!("  transparencyPercent: {}", fb.transparency_percent),
        "  # 是否置顶（true/false），默认 true".to_string(),
        format!("  alwaysOnTop: {}", fb.always_on_top),
        "  # 悬浮按钮窗口位置（左上角屏幕坐标），退出时自动保存；null 表示使用系统默认位置"
            .to_string(),
        "  position:".to_string(),
        format!("    x: {pos_x}"),
        format!("    y: {pos_y}"),
        String::new(),
        "# 人数选择窗口配置".to_string(),
        "pickCountDialog:".to_string(),
        "  # 是否默认播放喜庆点名音乐（true/false），默认 false".to_string(),
        format!("  defaultPlayMusic: {}", pick.default_play_music),
        "  # 背景变暗程度，范围 0-100（100 接近全黑），默认 50".to_string(),
        format!(
            "  backgroundDarknessPercent: {}",
            pick.background_darkness_percent
        ),
        "  # 人数默认值，范围 1-10 的整数，默认 1".to_string(),
        format!("  defaultCount: {}", pick.default_count),
        String::new(),
        "# 抽奖结果动画音效配置".to_string(),
        "pickResultDialog:".to_string(),
        "  # 是否默认播放抽奖音效（true/false），默认 true".to_string(),
        format!(
            "  defaultPlayGachaSound: {}",
            pick_result.default_play_gacha_sound
        ),
        "  # 抽奖音效音量（0.0-1.0），默认 0.6".to_string(),
        format!("  gachaSoundVolume: {}", pick_result.gacha_sound_volume),
        String::new(),
        "# 应用配置".to_string(),
        "webConfig:".to_string(),
        "  # 兼容旧版本字段；Tauri 版不再启动本地 Web 配置服务".to_string(),
        format!("  port: {}", web.port),
        "  # 启用管理员置顶增强（Windows 下会尝试管理员权限）".to_string(),
        format!("  adminTopmostEnabled: {}", web.admin_topmost_enabled),
        "  # 是否创建开机计划任务（管理员权限运行）".to_string(),
        format!("  adminAutoStartEnabled: {}", web.admin_auto_start_enabled),
        "  # 计划任务运行的可执行文件路径".to_string(),
        format!(
            "  adminAutoStartPath: \"{}\"",
            escape_yaml_string(&web.admin_auto_start_path)
        ),
        "  # 计划任务名称".to_string(),
        format!(
            "  adminAutoStartTaskName: \"{}\"",
            escape_yaml_string(&web.admin_auto_start_task_name)
        ),
        String::new(),
    ]
    .join("\n")
}

fn value_as_f64(value: Option<&Value>, fallback: f64) -> f64 {
    match value {
        Some(Value::Number(number)) => number.as_f64().unwrap_or(fallback),
        Some(Value::String(text)) => text.parse::<f64>().unwrap_or(fallback),
        _ => fallback,
    }
}

fn value_as_i32(value: Option<&Value>, fallback: i32) -> i32 {
    value_as_f64(value, fallback as f64).round() as i32
}

fn value_as_bool(value: Option<&Value>, fallback: bool) -> bool {
    match value {
        Some(Value::Bool(value)) => *value,
        Some(Value::String(text)) => match text.trim().to_ascii_lowercase().as_str() {
            "true" | "1" | "yes" => true,
            "false" | "0" | "no" => false,
            _ => fallback,
        },
        _ => fallback,
    }
}

fn value_as_string(value: Option<&Value>, fallback: &str) -> String {
    match value {
        Some(Value::String(text)) => text.clone(),
        Some(Value::Number(number)) => number.to_string(),
        Some(Value::Bool(value)) => value.to_string(),
        _ => fallback.to_string(),
    }
}

fn get_field<'a>(value: &'a Value, key: &str) -> Option<&'a Value> {
    value.as_object().and_then(|object| object.get(key))
}

fn normalize_config_value(value: Value) -> AppConfig {
    let default = AppConfig::default();
    let mut student_list = Vec::new();
    if let Some(Value::Array(students)) = get_field(&value, "studentList") {
        for item in students {
            match item {
                Value::String(name) => {
                    let trimmed = name.trim();
                    if !trimmed.is_empty() {
                        student_list.push(Student {
                            name: trimmed.to_string(),
                            weight: 1.0,
                        });
                    }
                }
                Value::Object(_) => {
                    let name = value_as_string(get_field(item, "name"), "")
                        .trim()
                        .to_string();
                    if !name.is_empty() {
                        student_list.push(Student {
                            name,
                            weight: value_as_f64(get_field(item, "weight"), 1.0),
                        });
                    }
                }
                _ => {}
            }
        }
    }

    let fb = get_field(&value, "floatingButton").unwrap_or(&Value::Null);
    let position = get_field(fb, "position").unwrap_or(&Value::Null);
    let pick = get_field(&value, "pickCountDialog").unwrap_or(&Value::Null);
    let pick_result = get_field(&value, "pickResultDialog").unwrap_or(&Value::Null);
    let web = get_field(&value, "webConfig").unwrap_or(&Value::Null);

    AppConfig {
        student_list,
        allow_repeat_draw: value_as_bool(
            get_field(&value, "allowRepeatDraw"),
            default.allow_repeat_draw,
        ),
        floating_button: FloatingButtonConfig {
            size_percent: clamp_f64(
                value_as_f64(
                    get_field(fb, "sizePercent"),
                    default.floating_button.size_percent,
                ),
                0.0,
                1000.0,
                default.floating_button.size_percent,
            ),
            transparency_percent: clamp_f64(
                value_as_f64(
                    get_field(fb, "transparencyPercent"),
                    default.floating_button.transparency_percent,
                ),
                0.0,
                100.0,
                default.floating_button.transparency_percent,
            ),
            always_on_top: value_as_bool(
                get_field(fb, "alwaysOnTop"),
                default.floating_button.always_on_top,
            ),
            position: FloatingPosition {
                x: match get_field(position, "x") {
                    Some(Value::Number(_)) | Some(Value::String(_)) => {
                        Some(value_as_i32(get_field(position, "x"), 0))
                    }
                    _ => None,
                },
                y: match get_field(position, "y") {
                    Some(Value::Number(_)) | Some(Value::String(_)) => {
                        Some(value_as_i32(get_field(position, "y"), 0))
                    }
                    _ => None,
                },
            },
        },
        pick_count_dialog: PickCountDialogConfig {
            default_play_music: value_as_bool(
                get_field(pick, "defaultPlayMusic"),
                default.pick_count_dialog.default_play_music,
            ),
            background_darkness_percent: clamp_f64(
                value_as_f64(
                    get_field(pick, "backgroundDarknessPercent"),
                    default.pick_count_dialog.background_darkness_percent,
                ),
                0.0,
                100.0,
                default.pick_count_dialog.background_darkness_percent,
            ),
            default_count: clamp_i32(
                value_as_i32(
                    get_field(pick, "defaultCount"),
                    default.pick_count_dialog.default_count,
                ),
                1,
                10,
                default.pick_count_dialog.default_count,
            ),
        },
        pick_result_dialog: PickResultDialogConfig {
            default_play_gacha_sound: value_as_bool(
                get_field(pick_result, "defaultPlayGachaSound"),
                default.pick_result_dialog.default_play_gacha_sound,
            ),
            gacha_sound_volume: clamp_f64(
                value_as_f64(
                    get_field(pick_result, "gachaSoundVolume"),
                    default.pick_result_dialog.gacha_sound_volume,
                ),
                0.0,
                1.0,
                default.pick_result_dialog.gacha_sound_volume,
            ),
        },
        web_config: WebConfig {
            port: clamp_i32(
                value_as_i32(get_field(web, "port"), default.web_config.port),
                1,
                65535,
                default.web_config.port,
            ),
            admin_topmost_enabled: value_as_bool(
                get_field(web, "adminTopmostEnabled"),
                default.web_config.admin_topmost_enabled,
            ),
            admin_auto_start_enabled: value_as_bool(
                get_field(web, "adminAutoStartEnabled"),
                default.web_config.admin_auto_start_enabled,
            ),
            admin_auto_start_path: value_as_string(
                get_field(web, "adminAutoStartPath"),
                &default.web_config.admin_auto_start_path,
            ),
            admin_auto_start_task_name: {
                let value = value_as_string(
                    get_field(web, "adminAutoStartTaskName"),
                    &default.web_config.admin_auto_start_task_name,
                );
                if value.trim().is_empty() {
                    default.web_config.admin_auto_start_task_name
                } else {
                    value.trim().to_string()
                }
            },
        },
    }
}

fn normalize_config(config: AppConfig) -> AppConfig {
    let value = serde_json::to_value(config).unwrap_or(Value::Null);
    normalize_config_value(value)
}

fn save_config(config: &AppConfig) -> Result<(), String> {
    let path = config_path()?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|error| format!("创建配置目录失败: {error}"))?;
    }
    fs::write(path, to_config_yaml_with_comments(config))
        .map_err(|error| format!("写入配置失败: {error}"))
}

fn write_default_config_if_missing(app: &AppHandle, path: &Path) -> Result<(), String> {
    if path.exists() {
        return Ok(());
    }

    if let Some(legacy_path) = legacy_config_path(app) {
        if legacy_path != path && legacy_path.exists() {
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent).map_err(|error| format!("创建配置目录失败: {error}"))?;
            }
            fs::copy(legacy_path, path).map_err(|error| format!("迁移旧配置失败: {error}"))?;
            return Ok(());
        }
    }

    save_config(&AppConfig::default())
}

fn load_config(app: &AppHandle) -> Result<AppConfig, String> {
    let path = config_path()?;
    write_default_config_if_missing(app, &path)?;
    let raw = fs::read_to_string(&path).map_err(|error| format!("读取配置失败: {error}"))?;
    let parsed: Value = serde_yaml::from_str(&raw).unwrap_or(Value::Null);
    let normalized = normalize_config_value(parsed);
    let normalized_raw = to_config_yaml_with_comments(&normalized);
    if raw != normalized_raw {
        fs::write(&path, normalized_raw).map_err(|error| format!("写入配置失败: {error}"))?;
    }
    Ok(normalized)
}

fn cached_config(state: &tauri::State<'_, AppState>) -> Result<AppConfig, String> {
    Ok(state
        .inner
        .lock()
        .map_err(|_| "状态锁定失败".to_string())?
        .config
        .clone())
}

fn push_log(app: &AppHandle, state: &tauri::State<'_, AppState>, level: &str, text: &str) {
    let entry = LogEntry {
        id: format!(
            "{}-{}",
            Utc::now().timestamp_millis(),
            rand::random::<u64>()
        ),
        level: level.to_string(),
        text: text.to_string(),
        time: Utc::now().to_rfc3339(),
    };
    if let Ok(mut guard) = state.inner.lock() {
        guard.logs.push_back(entry.clone());
        while guard.logs.len() > LOG_BUFFER_LIMIT {
            guard.logs.pop_front();
        }
    }
    let _ = app.emit("log-entry", entry);
}

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

fn apply_floating_window_config(window: &WebviewWindow, config: &AppConfig) {
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

fn create_floating_window(app: &AppHandle, config: &AppConfig) -> Result<WebviewWindow, String> {
    if let Some(window) = app.get_webview_window("floating") {
        apply_floating_window_config(&window, config);
        let _ = window.show();
        return Ok(window);
    }

    let (width, height) = floating_window_size(config);
    let mut builder = WebviewWindowBuilder::new(app, "floating", route_url(""))
        .title("Blue Random")
        .inner_size(width as f64, height as f64)
        .decorations(false)
        .resizable(false)
        .maximizable(false)
        .minimizable(false)
        .transparent(true)
        .shadow(false)
        .skip_taskbar(!cfg!(debug_assertions))
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

fn create_pick_count_window(app: &AppHandle) -> Result<WebviewWindow, String> {
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

fn create_pick_result_window(app: &AppHandle) -> Result<WebviewWindow, String> {
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

fn open_config_window(app: &AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("config") {
        let _ = window.show();
        let _ = window.set_focus();
        return Ok(());
    }

    WebviewWindowBuilder::new(app, "config", route_url("/config"))
        .title("Blue Random 配置")
        .inner_size(1120.0, 760.0)
        .resizable(true)
        .build()
        .map(|window| {
            let _ = window.set_focus();
        })
        .map_err(|error| format!("创建配置窗口失败: {error}"))
}

fn hide_floating_window(app: &AppHandle) {
    if let Some(window) = app.get_webview_window("floating") {
        let _ = window.hide();
    }
}

fn show_floating_window(app: &AppHandle) {
    if let Some(window) = app.get_webview_window("floating") {
        let _ = window.show();
        let _ = window.set_focus();
    }
}

fn persist_floating_position(app: &AppHandle, state: &tauri::State<'_, AppState>) {
    let Some(window) = app.get_webview_window("floating") else {
        return;
    };
    let Ok(position) = window.outer_position() else {
        return;
    };
    let Ok(guard) = state.inner.lock() else {
        return;
    };
    let mut config = guard.config.clone();
    drop(guard);
    config.floating_button.position.x = Some(position.x);
    config.floating_button.position.y = Some(position.y);
    if save_config(&config).is_ok() {
        if let Ok(mut guard) = state.inner.lock() {
            guard.config = config;
        }
    }
}

fn pick_students_by_weight(config: &AppConfig, count: i32) -> Vec<PickedStudent> {
    let pool = config
        .student_list
        .iter()
        .filter_map(|student| {
            let name = student.name.trim();
            if name.is_empty() {
                None
            } else {
                Some((name.to_string(), student.weight.max(0.0)))
            }
        })
        .collect::<Vec<_>>();

    if pool.is_empty() || count <= 0 {
        return Vec::new();
    }

    let target_count = count.max(0) as usize;
    let mut rng = rand::thread_rng();
    let mut picked = Vec::new();

    if config.allow_repeat_draw {
        let weighted_pool = pool
            .iter()
            .map(|(name, weight)| (name.clone(), weight.powf(WEIGHT_BOOST_GAMMA)))
            .collect::<Vec<_>>();
        let total_weight: f64 = weighted_pool.iter().map(|(_, weight)| *weight).sum();

        for _ in 0..target_count {
            let mut pick_index = None;
            if total_weight > 0.0 {
                let mut roll = rng.gen::<f64>() * total_weight;
                for (index, (_, weight)) in weighted_pool.iter().enumerate() {
                    roll -= *weight;
                    if roll <= 0.0 {
                        pick_index = Some(index);
                        break;
                    }
                }
            }
            let index = pick_index.unwrap_or_else(|| rng.gen_range(0..weighted_pool.len()));
            picked.push(PickedStudent {
                name: weighted_pool[index].0.clone(),
            });
        }
        return picked;
    }

    let mut positive_pool = pool
        .iter()
        .filter(|(_, weight)| *weight > 0.0)
        .map(|(name, weight)| {
            let random = rng.gen::<f64>().max(f64::MIN_POSITIVE);
            (name.clone(), -random.ln() / *weight)
        })
        .collect::<Vec<_>>();
    positive_pool.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));

    for (name, _) in positive_pool.into_iter().take(target_count) {
        picked.push(PickedStudent { name });
    }

    if picked.len() < target_count {
        let mut zero_pool = pool
            .iter()
            .filter(|(_, weight)| *weight <= 0.0)
            .map(|(name, _)| name.clone())
            .collect::<Vec<_>>();
        zero_pool.shuffle(&mut rng);
        for name in zero_pool.into_iter().take(target_count - picked.len()) {
            picked.push(PickedStudent { name });
        }
    }

    picked
}

fn quote_for_powershell(text: &str) -> String {
    text.replace('\'', "''")
}

#[cfg(target_os = "windows")]
fn is_process_elevated() -> bool {
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
fn is_process_elevated() -> bool {
    false
}

#[cfg(target_os = "windows")]
fn request_admin_relaunch() -> ApiResult {
    let exe_path = std::env::current_exe().unwrap_or_default();
    let exe_text = exe_path.to_string_lossy();
    let args = std::env::args()
        .skip(1)
        .map(|arg| format!("\"{}\"", arg.replace('"', "\\\"")))
        .collect::<Vec<_>>()
        .join(" ");
    let command = format!(
        "Start-Process -FilePath '{}' -ArgumentList '{}' -Verb RunAs",
        quote_for_powershell(&exe_text),
        quote_for_powershell(&args)
    );

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
fn request_admin_relaunch() -> ApiResult {
    ApiResult {
        ok: false,
        message: "当前系统不支持管理员提升。".to_string(),
        detail: None,
        restart_required: None,
    }
}

#[cfg(target_os = "windows")]
fn create_admin_startup_task_impl(task_name: &str, exe_path: &str) -> ApiResult {
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
fn create_admin_startup_task_impl(_task_name: &str, _exe_path: &str) -> ApiResult {
    ApiResult {
        ok: false,
        message: "仅支持 Windows 计划任务。".to_string(),
        detail: None,
        restart_required: None,
    }
}

fn parse_version_yaml(text: &str) -> serde_json::Map<String, Value> {
    serde_yaml::from_str::<Value>(text)
        .ok()
        .and_then(|value| value.as_object().cloned())
        .unwrap_or_default()
}

fn normalize_version(value: &str) -> Vec<i32> {
    value
        .trim()
        .trim_start_matches(['v', 'V'])
        .split('.')
        .filter_map(|part| part.parse::<i32>().ok())
        .collect()
}

fn compare_version(a: &str, b: &str) -> std::cmp::Ordering {
    let left = normalize_version(a);
    let right = normalize_version(b);
    let len = left.len().max(right.len());
    for index in 0..len {
        let av = *left.get(index).unwrap_or(&0);
        let bv = *right.get(index).unwrap_or(&0);
        match av.cmp(&bv) {
            std::cmp::Ordering::Equal => {}
            ordering => return ordering,
        }
    }
    std::cmp::Ordering::Equal
}

fn http_client() -> &'static reqwest::Client {
    HTTP_CLIENT.get_or_init(|| {
        reqwest::Client::builder()
            .user_agent("Blue-Random")
            .connect_timeout(Duration::from_secs(8))
            .timeout(Duration::from_secs(15))
            .build()
            .expect("failed to build update HTTP client")
    })
}

async fn fetch_text(
    client: &reqwest::Client,
    url: &str,
    accept: &str,
) -> Result<(u16, String), String> {
    let response = client
        .get(url)
        .header("Accept", accept)
        .send()
        .await
        .map_err(|error| error.to_string())?;
    let status = response.status().as_u16();
    let text = response.text().await.map_err(|error| error.to_string())?;
    Ok((status, text))
}

async fn check_update_from_main(local_version: &str) -> UpdateResult {
    let repo_owner = "Yun-Hydrogen";
    let repo_name = "ba_random_electron";
    let mut debug = Vec::new();
    let release_api =
        format!("https://api.github.com/repos/{repo_owner}/{repo_name}/releases/latest");
    debug.push(format!("GET {release_api}"));
    let client = http_client();

    let (release_status, release_body) =
        match fetch_text(client, &release_api, "application/vnd.github+json").await {
            Ok(response) => response,
            Err(error) => {
                return UpdateResult {
                    ok: false,
                    status: "error".to_string(),
                    title: "检查更新失败".to_string(),
                    detail: error,
                    commit_url: String::new(),
                    release_url: String::new(),
                    local_version: local_version.to_string(),
                    remote_version: String::new(),
                    debug,
                }
            }
        };

    if !(200..300).contains(&release_status) {
        return UpdateResult {
            ok: false,
            status: "error".to_string(),
            title: "检查更新失败".to_string(),
            detail: format!("Release 请求失败 ({release_status})"),
            commit_url: String::new(),
            release_url: String::new(),
            local_version: local_version.to_string(),
            remote_version: String::new(),
            debug,
        };
    }

    let release: Value = serde_json::from_str(&release_body).unwrap_or(Value::Null);
    let assets = release
        .get("assets")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();
    debug.push(format!("assets={}", assets.len()));
    let version_asset = assets.iter().find(|asset| {
        asset
            .get("name")
            .and_then(Value::as_str)
            .map(|name| name == "version.yml" || name.to_ascii_lowercase().ends_with("version.yml"))
            .unwrap_or(false)
    });
    let release_url = release
        .get("html_url")
        .and_then(Value::as_str)
        .unwrap_or("https://github.com/Yun-Hydrogen/ba_random_electron/releases/latest")
        .to_string();

    let Some(version_asset) = version_asset else {
        return UpdateResult {
            ok: false,
            status: "error".to_string(),
            title: "未找到版本描述文件".to_string(),
            detail: "发布中缺少 version.yml，请稍后再试。".to_string(),
            commit_url: String::new(),
            release_url,
            local_version: local_version.to_string(),
            remote_version: String::new(),
            debug,
        };
    };

    let Some(version_url) = version_asset
        .get("browser_download_url")
        .and_then(Value::as_str)
    else {
        return UpdateResult {
            ok: false,
            status: "error".to_string(),
            title: "未找到版本描述文件".to_string(),
            detail: "发布中缺少 version.yml 下载地址。".to_string(),
            commit_url: String::new(),
            release_url,
            local_version: local_version.to_string(),
            remote_version: String::new(),
            debug,
        };
    };

    debug.push(format!("GET {version_url}"));
    let (version_status, version_body) = match fetch_text(client, version_url, "text/plain").await {
        Ok(response) => response,
        Err(error) => {
            return UpdateResult {
                ok: false,
                status: "error".to_string(),
                title: "检查更新失败".to_string(),
                detail: error,
                commit_url: String::new(),
                release_url,
                local_version: local_version.to_string(),
                remote_version: String::new(),
                debug,
            }
        }
    };
    if !(200..300).contains(&version_status) {
        return UpdateResult {
            ok: false,
            status: "error".to_string(),
            title: "检查更新失败".to_string(),
            detail: format!("version.yml 下载失败 ({version_status})"),
            commit_url: String::new(),
            release_url,
            local_version: local_version.to_string(),
            remote_version: String::new(),
            debug,
        };
    }

    let remote_meta = parse_version_yaml(&version_body);
    let remote_version = remote_meta
        .get("version")
        .and_then(Value::as_str)
        .unwrap_or("0.0.0")
        .to_string();
    let remote_commit = remote_meta
        .get("commit")
        .and_then(Value::as_str)
        .unwrap_or("")
        .to_string();
    debug.push(format!("remoteVersion={remote_version}"));

    let mut commit_message = String::new();
    let mut commit_url = String::new();
    if !remote_commit.is_empty() {
        commit_url = format!("https://github.com/{repo_owner}/{repo_name}/commit/{remote_commit}");
        let commit_api = format!(
            "https://api.github.com/repos/{repo_owner}/{repo_name}/commits/{remote_commit}"
        );
        debug.push(format!("GET {commit_api}"));
        if let Ok((status, body)) =
            fetch_text(client, &commit_api, "application/vnd.github+json").await
        {
            if (200..300).contains(&status) {
                let commit_json: Value = serde_json::from_str(&body).unwrap_or(Value::Null);
                commit_message = commit_json
                    .pointer("/commit/message")
                    .and_then(Value::as_str)
                    .unwrap_or("")
                    .trim()
                    .to_string();
            }
        }
    }

    match compare_version(local_version, &remote_version) {
        std::cmp::Ordering::Less => UpdateResult {
            ok: true,
            status: "update".to_string(),
            title: format!("发现新版本：{remote_version}"),
            detail: if commit_message.is_empty() {
                "有新版本可用。".to_string()
            } else {
                format!("更新内容：\n{commit_message}")
            },
            commit_url,
            release_url,
            local_version: local_version.to_string(),
            remote_version,
            debug,
        },
        std::cmp::Ordering::Equal => UpdateResult {
            ok: true,
            status: "ok".to_string(),
            title: format!("已是最新版本：{local_version}"),
            detail: if commit_message.is_empty() {
                "无需更新。".to_string()
            } else {
                format!("当前提交：\n{commit_message}")
            },
            commit_url,
            release_url,
            local_version: local_version.to_string(),
            remote_version,
            debug,
        },
        std::cmp::Ordering::Greater => UpdateResult {
            ok: true,
            status: "easter".to_string(),
            title: format!("这是为什么呢？{local_version}"),
            detail: "为什么你的版本比最新发布的版本还要新呢？".to_string(),
            commit_url,
            release_url,
            local_version: local_version.to_string(),
            remote_version,
            debug,
        },
    }
}

#[tauri::command]
fn get_floating_button_config(
    state: tauri::State<'_, AppState>,
) -> Result<FloatingButtonConfig, String> {
    Ok(cached_config(&state)?.floating_button)
}

#[tauri::command]
fn floating_button_clicked(
    app: AppHandle,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    open_pick_count(app, state)
}

#[tauri::command]
fn floating_button_drag_start(
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
fn floating_button_drag_move(
    window: WebviewWindow,
    state: tauri::State<'_, AppState>,
    dx: f64,
    dy: f64,
) -> Result<(), String> {
    let session = state
        .inner
        .lock()
        .map_err(|_| "状态锁定失败".to_string())?
        .drag_session
        .clone();
    if let Some(session) = session {
        let next_x = session.start_x + dx.round() as i32;
        let next_y = session.start_y + dy.round() as i32;
        if (next_x - session.last_x).abs() < 2 && (next_y - session.last_y).abs() < 2 {
            return Ok(());
        }
        window
            .set_position(Position::Physical(PhysicalPosition {
                x: next_x,
                y: next_y,
            }))
            .map_err(|error| error.to_string())?;
        if let Ok(mut guard) = state.inner.lock() {
            if let Some(active) = &mut guard.drag_session {
                active.last_x = next_x;
                active.last_y = next_y;
            }
        }
    }
    Ok(())
}

#[tauri::command]
fn floating_button_drag_end(
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
fn floating_button_set_ignore_mouse(window: WebviewWindow, _ignore: bool) -> Result<(), String> {
    // Electron can ignore transparent pixels while forwarding mouse movement back to
    // the renderer. Tauri's cursor ignore API does not provide that forwarding mode,
    // so enabling it here makes the floating button impossible to click again.
    window
        .set_ignore_cursor_events(false)
        .map_err(|error| error.to_string())
}

#[tauri::command]
fn get_pick_count_config(
    state: tauri::State<'_, AppState>,
) -> Result<PickCountDialogConfig, String> {
    Ok(cached_config(&state)?.pick_count_dialog)
}

#[tauri::command]
fn open_pick_count(app: AppHandle, state: tauri::State<'_, AppState>) -> Result<(), String> {
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
fn cancel_pick_count(app: AppHandle, state: tauri::State<'_, AppState>) -> Result<(), String> {
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
fn confirm_pick_count(
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
    let config = cached_config(&state)?;
    let picked_students = pick_students_by_weight(&config, selected_count);
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

fn open_pick_result_window(
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

#[tauri::command]
fn get_pick_result_config(
    state: tauri::State<'_, AppState>,
) -> Result<PickResultDialogConfig, String> {
    Ok(cached_config(&state)?.pick_result_dialog)
}

#[tauri::command]
fn get_pick_results(state: tauri::State<'_, AppState>) -> Result<Vec<PickedStudent>, String> {
    Ok(state
        .inner
        .lock()
        .map_err(|_| "状态锁定失败".to_string())?
        .current_pick_results
        .clone())
}

#[tauri::command]
fn close_pick_result(app: AppHandle, state: tauri::State<'_, AppState>) -> Result<(), String> {
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
fn get_config(state: tauri::State<'_, AppState>) -> Result<AppConfig, String> {
    cached_config(&state)
}

#[tauri::command]
fn save_app_config(
    app: AppHandle,
    state: tauri::State<'_, AppState>,
    config: AppConfig,
) -> Result<ApiResult, String> {
    let normalized = normalize_config(config);
    save_config(&normalized)?;
    {
        state
            .inner
            .lock()
            .map_err(|_| "状态锁定失败".to_string())?
            .config = normalized.clone();
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
fn get_app_info(app: AppHandle) -> Result<AppInfo, String> {
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
async fn check_update(app: AppHandle) -> Result<UpdateResult, String> {
    Ok(check_update_from_main(&app.package_info().version.to_string()).await)
}

#[tauri::command]
fn request_admin_elevation(
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
fn create_admin_startup_task(
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
        state
            .inner
            .lock()
            .map_err(|_| "状态锁定失败".to_string())?
            .config = config;
    }
    Ok(result)
}

#[tauri::command]
fn renderer_log(app: AppHandle, state: tauri::State<'_, AppState>, level: String, text: String) {
    let level = if level.trim().is_empty() {
        "info"
    } else {
        level.trim()
    };
    push_log(&app, &state, level, &text);
}

#[tauri::command]
fn get_logs(state: tauri::State<'_, AppState>) -> Result<Vec<LogEntry>, String> {
    Ok(state
        .inner
        .lock()
        .map_err(|_| "状态锁定失败".to_string())?
        .logs
        .iter()
        .cloned()
        .collect())
}

fn setup_tray(app: &AppHandle) -> Result<(), String> {
    let config_item = MenuItem::with_id(app, "open_config", "配置", true, None::<&str>)
        .map_err(|e| e.to_string())?;
    let quit_item =
        MenuItem::with_id(app, "quit", "退出", true, None::<&str>).map_err(|e| e.to_string())?;
    let menu = Menu::with_items(app, &[&config_item, &quit_item]).map_err(|e| e.to_string())?;
    let mut builder = TrayIconBuilder::new()
        .tooltip("Blue Random")
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

pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let app_handle = app.handle().clone();
            let initial_config = load_config(&app_handle).unwrap_or_default();
            app.manage(AppState {
                inner: Mutex::new(RuntimeState::new(initial_config.clone())),
            });

            if initial_config.web_config.admin_topmost_enabled
                && cfg!(target_os = "windows")
                && !is_process_elevated()
            {
                let result = request_admin_relaunch();
                if result.ok {
                    app_handle.exit(0);
                    return Ok(());
                }
            }

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
            get_floating_button_config,
            floating_button_clicked,
            floating_button_drag_start,
            floating_button_drag_move,
            floating_button_drag_end,
            floating_button_set_ignore_mouse,
            get_pick_count_config,
            open_pick_count,
            cancel_pick_count,
            confirm_pick_count,
            get_pick_result_config,
            get_pick_results,
            close_pick_result,
            get_config,
            save_app_config,
            get_app_info,
            check_update,
            request_admin_elevation,
            create_admin_startup_task,
            renderer_log,
            get_logs
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
