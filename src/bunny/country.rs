use serde::Deserialize;
use url::Url;

/// Country struct returned by get_countries() function
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Country {
    /// Country name
    pub name: String,
    /// Country ISO code
    pub iso_code: String,
    /// Country is part of the EU
    #[serde(rename = "IsEU")]
    pub is_eu: bool,
    /// Tax rate in percentage
    pub tax_rate: f32,
    /// Tax prefix
    pub tax_prefix: String,
    /// URL to country flag
    pub flag_url: Url,
    /// ??
    pub pop_list: Vec<String>,
}
