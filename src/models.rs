use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PackProject {
    pub id: i64,
    pub name: String,
    pub authors: Vec<Author>,
    pub attachments: Vec<Attachment>,
    pub website_url: String,
    pub game_id: i64,
    pub summary: String,
    pub default_file_id: i64,
    pub download_count: f64,
    pub latest_files: Vec<LatestFile>,
    pub categories: Vec<Category>,
    pub status: i64,
    pub primary_category_id: i64,
    pub category_section: CategorySection,
    pub slug: String,
    pub game_version_latest_files: Vec<GameVersionLatestFile>,
    pub is_featured: bool,
    pub popularity_score: f64,
    pub game_popularity_rank: i64,
    pub primary_language: String,
    pub game_slug: String,
    pub game_name: String,
    pub portal_name: String,
    pub date_modified: String,
    pub date_created: String,
    pub date_released: String,
    pub is_available: bool,
    pub is_experiemental: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Author {
    pub name: String,
    pub url: String,
    pub project_id: i64,
    pub id: i64,
    pub project_title_id: Option<i64>,
    pub project_title_title: Option<String>,
    pub user_id: i64,
    pub twitch_id: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attachment {
    pub id: i64,
    pub project_id: i64,
    pub description: String,
    pub is_default: bool,
    pub thumbnail_url: String,
    pub title: String,
    pub url: String,
    pub status: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LatestFile {
    pub id: i64,
    pub display_name: String,
    pub file_name: String,
    pub file_date: String,
    pub file_length: i64,
    pub release_type: i64,
    pub file_status: i64,
    pub download_url: String,
    pub is_alternate: bool,
    pub alternate_file_id: i64,
    pub dependencies: Vec<::serde_json::Value>,
    pub is_available: bool,
    pub modules: Vec<Module>,
    pub package_fingerprint: i64,
    pub game_version: Vec<String>,
    pub sortable_game_version: Vec<SortableGameVersion>,
    pub install_metadata: ::serde_json::Value,
    pub changelog: ::serde_json::Value,
    pub has_install_script: bool,
    pub is_compatible_with_client: bool,
    pub category_section_package_type: i64,
    pub restrict_project_file_access: i64,
    pub project_status: i64,
    pub render_cache_id: i64,
    pub file_legacy_mapping_id: ::serde_json::Value,
    pub project_id: i64,
    pub parent_project_file_id: ::serde_json::Value,
    pub parent_file_legacy_mapping_id: ::serde_json::Value,
    pub file_type_id: ::serde_json::Value,
    pub expose_as_alternative: ::serde_json::Value,
    pub package_fingerprint_id: i64,
    pub game_version_date_released: String,
    pub game_version_mapping_id: i64,
    pub game_version_id: i64,
    pub game_id: i64,
    pub is_server_pack: bool,
    pub server_pack_file_id: Option<i64>,
    pub game_version_flavor: ::serde_json::Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Module {
    pub foldername: String,
    pub fingerprint: i64,
    #[serde(rename = "type")]
    pub type_field: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SortableGameVersion {
    pub game_version_padded: String,
    pub game_version: String,
    pub game_version_release_date: String,
    pub game_version_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Category {
    pub category_id: i64,
    pub name: String,
    pub url: String,
    pub avatar_url: String,
    pub parent_id: i64,
    pub root_id: i64,
    pub project_id: i64,
    pub avatar_id: i64,
    pub game_id: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CategorySection {
    pub id: i64,
    pub game_id: i64,
    pub name: String,
    pub package_type: i64,
    pub path: String,
    pub initial_inclusion_pattern: String,
    pub extra_include_pattern: ::serde_json::Value,
    pub game_category_id: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameVersionLatestFile {
    pub game_version: String,
    pub project_file_id: i64,
    pub project_file_name: String,
    pub file_type: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PackFileData {
    pub id: i64,
    pub display_name: String,
    pub file_name: String,
    pub file_date: String,
    pub file_length: i64,
    pub release_type: i64,
    pub file_status: i64,
    pub download_url: String,
    pub is_alternate: bool,
    pub alternate_file_id: i64,
    pub dependencies: Vec<::serde_json::Value>,
    pub is_available: bool,
    pub modules: Vec<PackFileModule>,
    pub package_fingerprint: i64,
    pub game_version: Vec<String>,
    pub install_metadata: ::serde_json::Value,
    pub server_pack_file_id: Option<i64>,
    pub has_install_script: bool,
    pub game_version_date_released: String,
    pub game_version_flavor: ::serde_json::Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PackFileModule {
    pub foldername: String,
    pub fingerprint: i64,
}
