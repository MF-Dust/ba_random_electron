use serde::{Deserialize, Serialize};

use crate::config::{PickCountDialogConfig, PickResultDialogConfig};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct PickedStudent {
    pub(crate) name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) avatar: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) academy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) club: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ApiResult {
    pub(crate) ok: bool,
    pub(crate) message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) detail: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) restart_required: Option<bool>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct AppInfo {
    pub(crate) version: String,
    pub(crate) is_debug_mode: bool,
    pub(crate) is_admin: bool,
    pub(crate) exe_path: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct UpdateResult {
    pub(crate) ok: bool,
    pub(crate) status: String,
    pub(crate) title: String,
    pub(crate) detail: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub(crate) commit_url: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub(crate) release_url: String,
    pub(crate) local_version: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub(crate) remote_version: String,
    pub(crate) debug: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct PickCountOpenPayload {
    pub(crate) config: PickCountDialogConfig,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct PickResultOpenPayload {
    pub(crate) token: u64,
    pub(crate) results: Vec<PickedStudent>,
    pub(crate) config: PickResultDialogConfig,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct PickResultResetPayload {
    pub(crate) token: u64,
    pub(crate) reason: String,
}
