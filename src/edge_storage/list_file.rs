use serde::Deserialize;

/// File information returned by list
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct ListFile {
    /// ??
    pub guid: String,
    /// Name of the storage zone the object is in
    pub storage_zone_name: String,
    /// Path to object
    pub path: String,
    /// Object name
    pub object_name: String,
    /// Length of the object in bytes
    pub length: u32,
    /// When the object was last modified
    pub last_changed: String,
    /// ??
    pub server_id: u32,
    /// ??
    pub array_number: u32,
    /// If the object is a directory
    pub is_directory: bool,
    /// ??
    pub user_id: String,
    /// Object content type
    pub content_type: String,
    /// When the object was created
    pub date_created: String,
    /// ID of the storage zone the object is in
    pub storage_zone_id: u32,
    /// File checksum on server
    pub checksum: String,
    /// Zones the object is replicated to
    pub replicated_zones: String,
}
