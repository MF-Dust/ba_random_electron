use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use tauri::{AppHandle, Manager};

use crate::utils::{clamp_f64, clamp_i32};
pub(crate) const ADMIN_TASK_DEFAULT_NAME: &str = "KVRandom (Admin)";
pub(crate) const MIN_PICK_COUNT: i32 = 1;
pub(crate) const MAX_PICK_COUNT: i32 = 10;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Student {
    pub(crate) name: String,
    pub(crate) weight: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct FloatingPosition {
    pub(crate) x: Option<i32>,
    pub(crate) y: Option<i32>,
}

impl Default for FloatingPosition {
    fn default() -> Self {
        Self { x: None, y: None }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct FloatingButtonConfig {
    pub(crate) size_percent: f64,
    pub(crate) transparency_percent: f64,
    pub(crate) always_on_top: bool,
    pub(crate) position: FloatingPosition,
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
pub(crate) struct PickCountDialogConfig {
    pub(crate) default_play_music: bool,
    pub(crate) background_darkness_percent: f64,
    pub(crate) default_count: i32,
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
pub(crate) struct PickResultDialogConfig {
    pub(crate) default_play_gacha_sound: bool,
    pub(crate) gacha_sound_volume: f64,
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
pub(crate) struct WebConfig {
    pub(crate) port: i32,
    pub(crate) admin_topmost_enabled: bool,
    pub(crate) admin_auto_start_enabled: bool,
    pub(crate) admin_auto_start_path: String,
    pub(crate) admin_auto_start_task_name: String,
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
pub(crate) struct AppConfig {
    pub(crate) student_list: Vec<Student>,
    pub(crate) allow_repeat_draw: bool,
    pub(crate) floating_button: FloatingButtonConfig,
    pub(crate) pick_count_dialog: PickCountDialogConfig,
    pub(crate) pick_result_dialog: PickResultDialogConfig,
    pub(crate) web_config: WebConfig,
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

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct StudentListParseResult {
    pub(crate) student_list: Vec<Student>,
    pub(crate) normalized_text: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ConfigFileSignature {
    pub(crate) modified: Option<SystemTime>,
    pub(crate) len: u64,
}

fn config_path() -> Result<PathBuf, String> {
    let current_dir =
        std::env::current_dir().map_err(|error| format!("获取当前目录失败啦: {error}"))?;
    if current_dir
        .file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| name == "src-tauri")
    {
        if let Some(project_dir) = current_dir.parent() {
            return Ok(project_dir.join("config.yml"));
        }
    }
    Ok(current_dir.join("config.yml"))
}

fn legacy_config_path(app: &AppHandle) -> Option<PathBuf> {
    app.path()
        .app_data_dir()
        .ok()
        .map(|dir| dir.join("config.yml"))
}

fn file_signature(path: &Path) -> Option<ConfigFileSignature> {
    let metadata = fs::metadata(path).ok()?;
    Some(ConfigFileSignature {
        modified: metadata.modified().ok(),
        len: metadata.len(),
    })
}

pub(crate) fn current_config_signature() -> Result<Option<ConfigFileSignature>, String> {
    Ok(file_signature(&config_path()?))
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
        "# 学生名单列表～".to_string(),
        format!("studentList:{student_lines}"),
        format!("allowRepeatDraw: {}", config.allow_repeat_draw),
        String::new(),
        "# 悬浮按钮配置～".to_string(),
        "floatingButton:".to_string(),
        "  # 按钮大小百分比（基准50px×50px），范围0-1000，默认100～".to_string(),
        format!("  sizePercent: {}", fb.size_percent),
        "  # 透明度百分比，范围0-100（0=完全不透明，100=完全透明），默认20～".to_string(),
        format!("  transparencyPercent: {}", fb.transparency_percent),
        "  # 是否置顶（true/false），默认true～".to_string(),
        format!("  alwaysOnTop: {}", fb.always_on_top),
        "  # 悬浮按钮窗口位置（左上角屏幕坐标），退出时自动保存；null表示使用默认位置～"
            .to_string(),
        "  position:".to_string(),
        format!("    x: {pos_x}"),
        format!("    y: {pos_y}"),
        String::new(),
        "# 人数选择窗口配置～".to_string(),
        "pickCountDialog:".to_string(),
        "  # 是否默认播放点名BGM（true/false），默认false～".to_string(),
        format!("  defaultPlayMusic: {}", pick.default_play_music),
        "  # 背景变暗程度，范围0-100（100接近全黑），默认50～".to_string(),
        format!(
            "  backgroundDarknessPercent: {}",
            pick.background_darkness_percent
        ),
        "  # 每次默认点名人数，范围1-10的整数，默认1～".to_string(),
        format!("  defaultCount: {}", pick.default_count),
        String::new(),
        "# 点名结果动画音效配置～".to_string(),
        "pickResultDialog:".to_string(),
        "  # 是否默认播放点名音效（true/false），默认true～".to_string(),
        format!(
            "  defaultPlayGachaSound: {}",
            pick_result.default_play_gacha_sound
        ),
        "  # 点名音效音量（0.0-1.0），默认0.6～".to_string(),
        format!("  gachaSoundVolume: {}", pick_result.gacha_sound_volume),
        String::new(),
        "# 应用配置～".to_string(),
        "webConfig:".to_string(),
        "  # 兼容旧版本字段；Tauri版不再启动本地Web配置服务～".to_string(),
        format!("  port: {}", web.port),
        "  # 启用管理员置顶增强（Windows下会尝试管理员权限）～".to_string(),
        format!("  adminTopmostEnabled: {}", web.admin_topmost_enabled),
        "  # 是否创建开机计划任务（管理员权限运行）～".to_string(),
        format!("  adminAutoStartEnabled: {}", web.admin_auto_start_enabled),
        "  # 开机任务运行的可执行文件路径～".to_string(),
        format!(
            "  adminAutoStartPath: \"{}\"",
            escape_yaml_string(&web.admin_auto_start_path)
        ),
        "  # 开机任务名称～".to_string(),
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

fn value_as_optional_i32(value: Option<&Value>) -> Option<i32> {
    match value {
        Some(Value::Number(number)) => number.as_f64().map(|value| value.round() as i32),
        Some(Value::String(text)) => {
            let trimmed = text.trim();
            if trimmed.is_empty() {
                None
            } else {
                trimmed
                    .parse::<f64>()
                    .ok()
                    .map(|value| value.round() as i32)
            }
        }
        _ => None,
    }
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

pub(crate) fn normalize_config_value(value: Value) -> AppConfig {
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
                x: value_as_optional_i32(get_field(position, "x")),
                y: value_as_optional_i32(get_field(position, "y")),
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
                MIN_PICK_COUNT,
                MAX_PICK_COUNT,
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

pub(crate) fn parse_student_list_text_impl(
    raw_text: &str,
    existing_students: &[Student],
) -> StudentListParseResult {
    let mut existing_weights = HashMap::with_capacity(existing_students.len());
    for student in existing_students {
        let name = student.name.trim();
        if !name.is_empty() {
            let weight = if student.weight.is_finite() {
                student.weight
            } else {
                1.0
            };
            existing_weights.insert(name.to_string(), weight);
        }
    }

    let mut seen = HashSet::new();
    let mut student_list = Vec::new();

    for name in raw_text
        .split(|char| char == '\n' || char == '\r' || char == ',')
        .map(str::trim)
        .filter(|name| !name.is_empty())
    {
        if seen.insert(name.to_string()) {
            student_list.push(Student {
                name: name.to_string(),
                weight: existing_weights.get(name).copied().unwrap_or(1.0),
            });
        }
    }

    let normalized_text = student_list
        .iter()
        .map(|student| student.name.as_str())
        .collect::<Vec<_>>()
        .join("\n");

    StudentListParseResult {
        student_list,
        normalized_text,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_config_value_uses_backend_defaults_for_missing_fields() {
        let result = normalize_config_value(Value::Null);

        assert!(result.student_list.is_empty());
        assert!(result.allow_repeat_draw);
        assert_eq!(result.floating_button.size_percent, 100.0);
        assert_eq!(result.floating_button.transparency_percent, 20.0);
        assert!(result.floating_button.always_on_top);
        assert_eq!(result.floating_button.position.x, None);
        assert_eq!(result.floating_button.position.y, None);
        assert!(!result.pick_count_dialog.default_play_music);
        assert_eq!(result.pick_count_dialog.background_darkness_percent, 50.0);
        assert_eq!(result.pick_count_dialog.default_count, MIN_PICK_COUNT);
        assert!(result.pick_result_dialog.default_play_gacha_sound);
        assert_eq!(result.pick_result_dialog.gacha_sound_volume, 0.6);
        assert_eq!(result.web_config.port, 21219);
        assert_eq!(
            result.web_config.admin_auto_start_task_name,
            ADMIN_TASK_DEFAULT_NAME
        );
    }

    #[test]
    fn normalize_config_value_clamps_numeric_ranges() {
        let result = normalize_config_value(serde_json::json!({
            "floatingButton": {
                "sizePercent": -1,
                "transparencyPercent": 150
            },
            "pickCountDialog": {
                "backgroundDarknessPercent": -20,
                "defaultCount": 99
            },
            "pickResultDialog": {
                "gachaSoundVolume": 2
            },
            "webConfig": {
                "port": 70000
            }
        }));

        assert_eq!(result.floating_button.size_percent, 0.0);
        assert_eq!(result.floating_button.transparency_percent, 100.0);
        assert_eq!(result.pick_count_dialog.background_darkness_percent, 0.0);
        assert_eq!(result.pick_count_dialog.default_count, MAX_PICK_COUNT);
        assert_eq!(result.pick_result_dialog.gacha_sound_volume, 1.0);
        assert_eq!(result.web_config.port, 65535);
    }

    #[test]
    fn normalize_config_value_clamps_pick_count_to_minimum() {
        let result = normalize_config_value(serde_json::json!({
            "pickCountDialog": {
                "defaultCount": -5
            }
        }));

        assert_eq!(result.pick_count_dialog.default_count, MIN_PICK_COUNT);
    }

    #[test]
    fn parse_student_list_text_dedupes_and_preserves_weights() {
        let existing_students = vec![
            Student {
                name: "Alice".to_string(),
                weight: 1.7,
            },
            Student {
                name: "Bob".to_string(),
                weight: 0.4,
            },
        ];

        let result =
            parse_student_list_text_impl(" Alice\r\nBob, Charlie\nAlice\n\n", &existing_students);

        assert_eq!(result.normalized_text, "Alice\nBob\nCharlie");
        assert_eq!(result.student_list.len(), 3);
        assert_eq!(result.student_list[0].name, "Alice");
        assert_eq!(result.student_list[0].weight, 1.7);
        assert_eq!(result.student_list[1].name, "Bob");
        assert_eq!(result.student_list[1].weight, 0.4);
        assert_eq!(result.student_list[2].name, "Charlie");
        assert_eq!(result.student_list[2].weight, 1.0);
    }
}

pub(crate) fn save_config(config: &AppConfig) -> Result<(), String> {
    let path = config_path()?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|error| format!("创建配置目录失败啦: {error}"))?;
    }
    fs::write(path, to_config_yaml_with_comments(config))
        .map_err(|error| format!("写入配置失败啦: {error}"))
}

fn write_default_config_if_missing(app: &AppHandle, path: &Path) -> Result<(), String> {
    if path.exists() {
        return Ok(());
    }

    if let Some(legacy_path) = legacy_config_path(app) {
        if legacy_path != path && legacy_path.exists() {
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent)
                    .map_err(|error| format!("创建配置目录失败啦: {error}"))?;
            }
            fs::copy(legacy_path, path).map_err(|error| format!("迁移旧配置失败啦: {error}"))?;
            return Ok(());
        }
    }

    save_config(&AppConfig::default())
}

pub(crate) fn load_config(app: &AppHandle) -> Result<AppConfig, String> {
    let path = config_path()?;
    write_default_config_if_missing(app, &path)?;
    let raw = fs::read_to_string(&path).map_err(|error| format!("读取配置失败啦: {error}"))?;
    let parsed: Value = serde_yaml::from_str(&raw).unwrap_or(Value::Null);
    let normalized = normalize_config_value(parsed);
    let normalized_raw = to_config_yaml_with_comments(&normalized);
    if raw != normalized_raw {
        fs::write(&path, normalized_raw).map_err(|error| format!("写入配置失败啦: {error}"))?;
    }
    Ok(normalized)
}

pub(crate) fn load_config_with_signature(
    app: &AppHandle,
) -> Result<(AppConfig, Option<ConfigFileSignature>), String> {
    let config = load_config(app)?;
    Ok((config, current_config_signature()?))
}
