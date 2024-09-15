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
    pub series_id: i64,
    pub episode_id: i64,
    pub season_number: i64,
    pub series: Option<Series>,
    pub episode: Option<Episode>,
    pub languages: Option<Vec<Language>>,
    pub quality: Option<Quality>,
    pub custom_formats: Option<Vec<CustomFormat>>,
    pub custom_format_score: Option<i64>,
    pub size: i64,
    pub title: String,
    pub sizeleft: i64,
    pub timeleft: String,
    pub estimated_completion_time: Option<String>,
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
    pub episode_has_file: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Series {
    pub id: i64,
    pub title: String,
    pub alternate_titles: Option<Vec<AlternateTitle>>,
    pub sort_title: String,
    pub status: String,
    pub ended: bool,
    pub profile_name: Option<String>,
    pub overview: String,
    pub next_airing: Option<String>,
    pub previous_airing: Option<String>,
    pub network: Option<String>,
    pub air_time: String,
    pub images: Vec<Image>,
    pub original_language: Language,
    pub remote_poster: Option<String>,
    pub seasons: Vec<Season>,
    pub year: f64,
    pub path: String,
    pub quality_profile_id: i64,
    pub season_folder: bool,
    pub monitored: bool,
    pub monitor_new_items: String,
    pub use_scene_numbering: bool,
    pub runtime: i64,
    pub tvdb_id: i64,
    pub tv_rage_id: i64,
    pub tv_maze_id: i64,
    pub tmdb_id: i64,
    pub first_aired: String,
    pub last_aired: String,
    pub series_type: String,
    pub clean_title: String,
    pub imdb_id: String,
    pub title_slug: String,
    pub root_folder_path: Option<String>,
    pub folder: Option<String>,
    pub certification: String,
    pub genres: Vec<String>,
    pub tags: Vec<i64>,
    pub added: String,
    pub add_options: Option<AddOptions>,
    pub ratings: Ratings,
    pub statistics: Option<Statistics>,
    pub episodes_changed: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlternateTitle {
    pub title: String,
    pub season_number: i64,
    pub scene_season_number: i64,
    pub scene_origin: String,
    pub comment: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Image {
    pub cover_type: String,
    pub url: Option<String>,
    pub remote_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Season {
    pub season_number: i64,
    pub monitored: bool,
    pub statistics: Option<Statistics>,
    pub images: Option<Vec<Image>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Statistics {
    pub next_airing: String,
    pub previous_airing: String,
    pub episode_file_count: i64,
    pub episode_count: i64,
    pub total_episode_count: i64,
    pub size_on_disk: i64,
    pub release_groups: Vec<String>,
    pub percent_of_episodes: i64,
    pub season_count: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddOptions {
    pub ignore_episodes_with_files: bool,
    pub ignore_episodes_without_files: bool,
    pub monitor: String,
    pub search_for_missing_episodes: bool,
    pub search_for_cutoff_unmet_episodes: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ratings {
    pub votes: i64,
    pub value: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Episode {
    pub id: i64,
    pub series_id: i64,
    pub tvdb_id: i64,
    pub episode_file_id: Option<i64>,
    pub season_number: i64,
    pub episode_number: i64,
    pub title: Option<String>,
    pub air_date: String,
    pub air_date_utc: String,
    pub runtime: f64,
    pub finale_type: Option<String>,
    pub overview: String,
    pub episode_file: Option<EpisodeFile>,
    pub has_file: bool,
    pub monitored: bool,
    pub absolute_episode_number: Option<i64>,
    pub scene_absolute_episode_number: Option<i64>,
    pub scene_episode_number: Option<i64>,
    pub scene_season_number: Option<i64>,
    pub unverified_scene_numbering: Option<bool>,
    pub end_time: Option<String>,
    pub grab_date: Option<String>,
    pub series: Option<Series>,
    pub images: Option<Vec<Image>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EpisodeFile {
    pub id: i64,
    pub series_id: i64,
    pub season_number: i64,
    pub relative_path: String,
    pub path: String,
    pub size: f64,
    pub date_added: String,
    pub scene_name: String,
    pub release_group: String,
    pub languages: Vec<Language>,
    pub quality: Quality,
    pub custom_formats: Vec<CustomFormat>,
    pub custom_format_score: i64,
    pub indexer_flags: i64,
    pub release_type: String,
    pub media_info: MediaInfo,
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
    pub quality: QualityDetail,
    pub revision: Revision,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QualityDetail {
    pub id: i64,
    pub name: String,
    pub source: String,
    pub resolution: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Revision {
    pub version: f64,
    pub real: f64,
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
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaInfo {
    pub id: Option<i64>,
    pub audio_bitrate: i64,
    pub audio_channels: f64,
    pub audio_codec: String,
    pub audio_languages: String,
    pub audio_stream_count: i64,
    pub video_bit_depth: f64,
    pub video_bitrate: f64,
    pub video_codec: String,
    pub video_fps: f64,
    pub video_dynamic_range: String,
    pub video_dynamic_range_type: String,
    pub resolution: String,
    pub run_time: String,
    pub scan_type: String,
    pub subtitles: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatusMessage {
    pub title: String,
    pub messages: Vec<String>,
}
