use serde::Deserialize;

/// Pagination struct used by Bunny.net API
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Pagination<T> {
    /// Vector of type T
    pub items: Vec<T>,
    /// Current page number
    pub current_page: i32,
    /// Total amount of type T
    pub total_items: i32,
    /// Has more items
    pub has_more_items: bool,
}
