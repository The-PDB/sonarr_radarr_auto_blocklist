use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Queue {
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
    pub id: i64,
    pub movie_id: i64,
    pub movie: Option<Movie>,
    pub languages: Option<Vec<Language>>,
    pub quality: Option<Quality>,
    pub custom_formats: Option<Vec<CustomFormat>>,
    pub custom_format_score: Option<i64>,
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
    pub output_path: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Movie {
    pub id: i64,
    pub title: String,
    pub original_title: String,
    pub original_language: Language,
    pub alternate_titles: Vec<AlternateTitle>,
    pub secondary_year: i64,
    pub secondary_year_source_id: i64,
    pub sort_title: String,
    pub size_on_disk: i64,
    pub status: String,
    pub overview: String,
    pub in_cinemas: String,
    pub physical_release: String,
    pub digital_release: String,
    pub release_date: String,
    pub physical_release_note: String,
    pub images: Vec<Image>,
    pub website: String,
    pub remote_poster: String,
    pub year: i64,
    pub you_tube_trailer_id: String,
    pub studio: String,
    pub path: String,
    pub quality_profile_id: i64,
    pub has_file: bool,
    pub movie_file_id: i64,
    pub monitored: bool,
    pub minimum_availability: String,
    pub is_available: bool,
    pub folder_name: String,
    pub runtime: i64,
    pub clean_title: String,
    pub imdb_id: String,
    pub tmdb_id: i64,
    pub title_slug: String,
    pub root_folder_path: String,
    pub folder: String,
    pub certification: String,
    pub genres: Vec<String>,
    pub tags: Vec<i64>,
    pub added: String,
    pub add_options: AddOptions,
    pub ratings: Ratings,
    pub movie_file: MovieFile,
    pub collection: Collection,
    pub popularity: i64,
    pub statistics: Statistics,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlternateTitle {
    pub id: i64,
    pub source_type: String,
    pub movie_metadata_id: i64,
    pub title: String,
    pub clean_title: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Image {
    pub cover_type: String,
    pub url: String,
    pub remote_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddOptions {
    pub ignore_episodes_with_files: bool,
    pub ignore_episodes_without_files: bool,
    pub monitor: String,
    pub search_for_movie: bool,
    pub add_method: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ratings {
    pub imdb: Imdb,
    pub tmdb: Tmdb,
    pub metacritic: Metacritic,
    pub rotten_tomatoes: RottenTomatoes,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Imdb {
    pub votes: i64,
    pub value: i64,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tmdb {
    pub votes: i64,
    pub value: i64,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metacritic {
    pub votes: i64,
    pub value: i64,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RottenTomatoes {
    pub votes: i64,
    pub value: i64,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MovieFile {
    pub id: i64,
    pub movie_id: i64,
    pub relative_path: String,
    pub path: String,
    pub size: i64,
    pub date_added: String,
    pub scene_name: String,
    pub release_group: String,
    pub edition: String,
    pub languages: Vec<Language>,
    pub quality: Quality,
    pub custom_formats: Vec<CustomFormat>,
    pub custom_format_score: i64,
    pub indexer_flags: i64,
    pub media_info: MediaInfo,
    pub original_file_path: String,
    pub quality_cutoff_not_met: bool,
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
    pub modifier: String,
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
    pub include_custom_format_when_renaming: Option<bool>,
    pub specifications: Option<Vec<Specification>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Specification {
    pub id: i64,
    pub name: String,
    pub implementation: String,
    pub implementation_name: String,
    pub info_link: String,
    pub negate: bool,
    pub required: bool,
    pub fields: Vec<Field>,
    pub presets: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Field {
    pub order: i64,
    pub name: String,
    pub label: String,
    pub unit: String,
    pub help_text: String,
    pub help_text_warning: String,
    pub help_link: String,
    pub value: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub advanced: bool,
    pub select_options: Vec<SelectOption>,
    pub select_options_provider_action: String,
    pub section: String,
    pub hidden: String,
    pub privacy: String,
    pub placeholder: String,
    pub is_float: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SelectOption {
    pub value: i64,
    pub name: String,
    pub order: i64,
    pub hint: String,
    pub divider_after: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaInfo {
    pub id: i64,
    pub audio_bitrate: i64,
    pub audio_channels: i64,
    pub audio_codec: String,
    pub audio_languages: String,
    pub audio_stream_count: i64,
    pub video_bit_depth: i64,
    pub video_bitrate: i64,
    pub video_codec: String,
    pub video_fps: i64,
    pub video_dynamic_range: String,
    pub video_dynamic_range_type: String,
    pub resolution: String,
    pub run_time: String,
    pub scan_type: String,
    pub subtitles: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Collection {
    pub title: String,
    pub tmdb_id: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Statistics {
    pub movie_file_count: i64,
    pub size_on_disk: i64,
    pub release_groups: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatusMessage {
    pub title: String,
    pub messages: Vec<String>,
}
