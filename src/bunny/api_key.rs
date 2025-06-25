use serde::Deserialize;

/// API Key struct returned by list_api_keys()
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct ApiKey {
    /// API Key ID
    pub id: i32,
    /// API Key
    pub key: String,
    /// ??
    pub roles: Vec<String>,
}
