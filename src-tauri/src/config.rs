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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) avatar: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) academy: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) club: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct FloatingPosition {
    pub(crate) x: Option<i32>,
    pub(crate) y: Option<i32>,
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
pub(crate) struct FileSignature {
    pub(crate) modified: Option<SystemTime>,
    pub(crate) len: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ConfigFileSignature {
    pub(crate) config: Option<FileSignature>,
    pub(crate) list: Option<FileSignature>,
}

fn app_config_dir(app: &AppHandle) -> Result<PathBuf, String> {
    app.path()
        .app_config_dir()
        .or_else(|_| app.path().app_data_dir())
        .map_err(|error| format!("获取配置目录失败啦: {error}"))
}

fn legacy_run_dir() -> Option<PathBuf> {
    let current_dir = std::env::current_dir().ok()?;
    if current_dir
        .file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| name == "src-tauri")
    {
        if let Some(project_dir) = current_dir.parent() {
            return Some(project_dir.to_path_buf());
        }
    }
    Some(current_dir)
}

fn config_path(app: &AppHandle) -> Result<PathBuf, String> {
    Ok(app_config_dir(app)?.join("config.yml"))
}

pub(crate) fn list_path(app: &AppHandle) -> Result<PathBuf, String> {
    Ok(app_config_dir(app)?.join("list.yaml"))
}

fn to_list_yaml_with_comments(students: &[Student]) -> String {
    let mut lines = vec![
        "# 学生名单～".to_string(),
        "# 支持的字段: name(姓名), weight(权重), avatar(立绘路径), academy(学院), club(社团)"
            .to_string(),
        String::new(),
    ];
    if students.is_empty() {
        lines.push("students: []".to_string());
    } else {
        lines.push("students:".to_string());
        for student in students {
            lines.push(format!(
                "  - name: \"{}\"",
                escape_yaml_string(&student.name)
            ));
            lines.push(format!("    weight: {}", student.weight));
            if let Some(avatar) = &student.avatar {
                lines.push(format!("    avatar: \"{}\"", escape_yaml_string(avatar)));
            }
            if let Some(academy) = &student.academy {
                lines.push(format!("    academy: \"{}\"", escape_yaml_string(academy)));
            }
            if let Some(club) = &student.club {
                lines.push(format!("    club: \"{}\"", escape_yaml_string(club)));
            }
        }
    }
    lines.push(String::new());
    lines.join("\n")
}

pub(crate) fn load_student_list(app: &AppHandle) -> Result<Vec<Student>, String> {
    let path = list_path(app)?;
    if !path.exists() {
        return Ok(Vec::new());
    }
    let raw = fs::read_to_string(&path).map_err(|e| format!("读取名单失败啦: {e}"))?;
    let parsed: Value = serde_yaml::from_str(&raw).map_err(|e| format!("解析名单失败啦: {e}"))?;
    let mut students = Vec::new();
    if let Some(Value::Array(items)) = get_field(&parsed, "students") {
        for item in items {
            if let Value::Object(_) = item {
                let name = value_as_string(get_field(item, "name"), "")
                    .trim()
                    .to_string();
                if !name.is_empty() {
                    let avatar = match get_field(item, "avatar") {
                        Some(Value::String(s)) if !s.trim().is_empty() => {
                            Some(s.trim().to_string())
                        }
                        _ => None,
                    };
                    let academy = match get_field(item, "academy") {
                        Some(Value::String(s)) if !s.trim().is_empty() => {
                            Some(s.trim().to_string())
                        }
                        _ => None,
                    };
                    let club = match get_field(item, "club") {
                        Some(Value::String(s)) if !s.trim().is_empty() => {
                            Some(s.trim().to_string())
                        }
                        _ => None,
                    };
                    students.push(Student {
                        name,
                        weight: value_as_f64(get_field(item, "weight"), 1.0),
                        avatar,
                        academy,
                        club,
                    });
                }
            }
        }
    }
    Ok(students)
}

pub(crate) fn save_student_list(app: &AppHandle, students: &[Student]) -> Result<(), String> {
    let path = list_path(app)?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("创建目录失败啦: {e}"))?;
    }
    fs::write(path, to_list_yaml_with_comments(students))
        .map_err(|e| format!("写入名单失败啦: {e}"))
}

fn legacy_paths(app: &AppHandle, file_name: &str, target: &Path) -> Vec<PathBuf> {
    let mut paths = Vec::new();
    if let Ok(app_data_dir) = app.path().app_data_dir() {
        paths.push(app_data_dir.join(file_name));
    }
    if let Some(run_dir) = legacy_run_dir() {
        paths.push(run_dir.join(file_name));
    }
    paths
        .into_iter()
        .filter(|path| path != target && path.exists())
        .collect()
}

fn copy_legacy_file_if_missing(
    app: &AppHandle,
    target: &Path,
    file_name: &str,
) -> Result<bool, String> {
    if target.exists() {
        return Ok(false);
    }
    let Some(legacy_path) = legacy_paths(app, file_name, target).into_iter().next() else {
        return Ok(false);
    };
    if let Some(parent) = target.parent() {
        fs::create_dir_all(parent).map_err(|error| format!("创建配置目录失败啦: {error}"))?;
    }
    fs::copy(&legacy_path, target).map_err(|error| format!("迁移旧文件失败啦: {error}"))?;
    Ok(true)
}

pub(crate) fn migrate_student_list_if_needed(app: &AppHandle) -> Result<(), String> {
    let list_file = list_path(app)?;
    if list_file.exists() {
        return Ok(());
    }
    if copy_legacy_file_if_missing(app, &list_file, "list.yaml")? {
        return Ok(());
    }
    let cfg_path = config_path(app)?;
    if !cfg_path.exists() {
        return Ok(());
    }
    let raw = fs::read_to_string(&cfg_path).map_err(|e| format!("读取配置失败啦: {e}"))?;
    let parsed: Value = serde_yaml::from_str(&raw).map_err(|e| format!("解析配置失败啦: {e}"))?;
    let config = normalize_config_value(parsed);
    if config.student_list.is_empty() {
        return Ok(());
    }
    save_student_list(app, &config.student_list)?;
    let mut cleared = config;
    cleared.student_list = Vec::new();
    save_config(app, &cleared)?;
    Ok(())
}

fn file_signature(path: &Path) -> Option<FileSignature> {
    let metadata = fs::metadata(path).ok()?;
    Some(FileSignature {
        modified: metadata.modified().ok(),
        len: metadata.len(),
    })
}

pub(crate) fn current_config_signature(
    app: &AppHandle,
) -> Result<Option<ConfigFileSignature>, String> {
    let signature = ConfigFileSignature {
        config: file_signature(&config_path(app)?),
        list: file_signature(&list_path(app)?),
    };
    if signature.config.is_none() && signature.list.is_none() {
        Ok(None)
    } else {
        Ok(Some(signature))
    }
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
    [
        "# 学生名单已拆分到 list.yaml；这里保留空字段用于兼容旧版本～".to_string(),
        "studentList: []".to_string(),
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
                            avatar: None,
                            academy: None,
                            club: None,
                        });
                    }
                }
                Value::Object(_) => {
                    let name = value_as_string(get_field(item, "name"), "")
                        .trim()
                        .to_string();
                    if !name.is_empty() {
                        let avatar = match get_field(item, "avatar") {
                            Some(Value::String(s)) if !s.trim().is_empty() => {
                                Some(s.trim().to_string())
                            }
                            _ => None,
                        };
                        let academy = match get_field(item, "academy") {
                            Some(Value::String(s)) if !s.trim().is_empty() => {
                                Some(s.trim().to_string())
                            }
                            _ => None,
                        };
                        let club = match get_field(item, "club") {
                            Some(Value::String(s)) if !s.trim().is_empty() => {
                                Some(s.trim().to_string())
                            }
                            _ => None,
                        };
                        student_list.push(Student {
                            name,
                            weight: value_as_f64(get_field(item, "weight"), 1.0),
                            avatar,
                            academy,
                            club,
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
    let mut existing_map = HashMap::with_capacity(existing_students.len());
    for student in existing_students {
        let name = student.name.trim();
        if !name.is_empty() {
            existing_map.insert(name.to_string(), student.clone());
        }
    }

    let mut seen = HashSet::new();
    let mut student_list = Vec::new();

    for name in raw_text
        .split(['\n', '\r', ','])
        .map(str::trim)
        .filter(|name| !name.is_empty())
    {
        if seen.insert(name.to_string()) {
            if let Some(existing) = existing_map.get(name) {
                let weight = if existing.weight.is_finite() {
                    existing.weight
                } else {
                    1.0
                };
                student_list.push(Student {
                    name: name.to_string(),
                    weight,
                    avatar: existing.avatar.clone(),
                    academy: existing.academy.clone(),
                    club: existing.club.clone(),
                });
            } else {
                student_list.push(Student {
                    name: name.to_string(),
                    weight: 1.0,
                    avatar: None,
                    academy: None,
                    club: None,
                });
            }
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
                avatar: Some("/img/alice.png".to_string()),
                academy: Some("Abydos".to_string()),
                club: None,
            },
            Student {
                name: "Bob".to_string(),
                weight: 0.4,
                avatar: None,
                academy: None,
                club: None,
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

pub(crate) fn save_config(app: &AppHandle, config: &AppConfig) -> Result<(), String> {
    let path = config_path(app)?;
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

    if copy_legacy_file_if_missing(app, path, "config.yml")? {
        return Ok(());
    }

    save_config(app, &AppConfig::default())
}

pub(crate) fn load_config(app: &AppHandle) -> Result<AppConfig, String> {
    let path = config_path(app)?;
    write_default_config_if_missing(app, &path)?;
    migrate_student_list_if_needed(app)?;
    let raw = fs::read_to_string(&path).map_err(|error| format!("读取配置失败啦: {error}"))?;
    let parsed: Value =
        serde_yaml::from_str(&raw).map_err(|error| format!("解析配置失败啦: {error}"))?;
    let mut normalized = normalize_config_value(parsed);
    let list_file_exists = list_path(app)?.exists();
    let list_students = load_student_list(app)?;
    if list_file_exists {
        normalized.student_list = list_students;
    }
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
    Ok((config, current_config_signature(app)?))
}
