use serde::Deserialize;

/// Region struct returned by region_list()
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Region {
    /// Region ID
    pub id: i32,
    /// Name of the region
    pub name: String,
    /// Price per gigabyte in region
    pub price_per_gigabyte: f32,
    /// Region 2 letter code
    pub region_code: String,
    /// Continent 2 letter code
    pub continent_code: String,
    /// Country 2 letter code
    pub country_code: String,
    /// Region latitude
    pub latitude: f32,
    /// Region longitude
    pub longitude: f32,
    /// ??
    pub allow_latency_routing: bool,
}
