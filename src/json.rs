use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SonarrQueue {
    pub page: i64,
    pub page_size: i64,
    pub sort_key: String,
    pub sort_direction: String,
    pub total_records: i64,
    pub records: Vec<Record>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Record {
    pub series_id: i64,
    pub episode_id: i64,
    pub season_number: i64,
    pub languages: Vec<Language>,
    pub quality: Quality,
    pub custom_formats: Vec<CustomFormat>,
    pub custom_format_score: i64,
    pub size: i64,
    pub title: String,
    pub sizeleft: i64,
    pub timeleft: String,
    pub estimated_completion_time: String,
    pub added: String,
    pub status: String,
    pub tracked_download_status: String,
    pub tracked_download_state: String,
    pub status_messages: Vec<StatusMessage>,
    pub error_message: Option<String>,
    pub download_id: String,
    pub protocol: String,
    pub download_client: String,
    pub download_client_has_post_import_category: bool,
    pub indexer: String,
    pub episode_has_file: bool,
    pub id: i64,
    pub output_path: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Language {
    pub id: i64,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Quality {
    pub quality: Quality2,
    pub revision: Revision,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Quality2 {
    pub id: i64,
    pub name: String,
    pub source: String,
    pub resolution: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Revision {
    pub version: i64,
    pub real: i64,
    pub is_repack: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomFormat {
    pub id: i64,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatusMessage {
    pub title: String,
    pub messages: Vec<String>,
}
