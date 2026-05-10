use serde_json::Value;
use std::sync::OnceLock;
use std::time::Duration;

use crate::models::UpdateResult;

static HTTP_CLIENT: OnceLock<reqwest::Client> = OnceLock::new();
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

pub(crate) async fn check_update_from_main(local_version: &str) -> UpdateResult {
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
