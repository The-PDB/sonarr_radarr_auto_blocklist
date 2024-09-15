use core::fmt;

use anyhow::{ensure, Context, Result};
use reqwest::{header::ACCEPT, Client};

use crate::json::{radarr, sonarr};

pub struct Api {
    source_url: String,
    api_key: String,
    radarr: bool,
    skip_redownload: bool,

    client: Client,
}

impl Api {
    const API_PATH: &'static str = "api/v3";
    const QUEUE_PARAMS: &'static str = "page=1&pageSize=1000";
    const DELETE_PARAMS: &'static str =
        "removeFromClient=false&blocklist=true&changeCategory=false";

    pub fn new(source_url: String, api_key: String, radarr: bool, skip_redownload: bool) -> Self {
        Self {
            source_url,
            api_key,
            radarr,
            skip_redownload,
            client: Client::new(),
        }
    }

    pub async fn get_queue(&self) -> Result<Box<dyn QueueJson>> {
        let url = format!(
            "{}/{}/queue?{}&apikey={}",
            self.source_url,
            Self::API_PATH,
            Self::QUEUE_PARAMS,
            self.api_key
        );

        let response = self
            .client
            .get(&url)
            .header(ACCEPT, "application/json")
            .send()
            .await
            .context("failed to get queue data")?;

        ensure!(
            response.status().is_success(),
            "Failed to get queue data. Bad status code: {}",
            response.status()
        );

        if self.radarr {
            Ok(Box::new(response.json::<radarr::Queue>().await?))
        } else {
            Ok(Box::new(response.json::<sonarr::Queue>().await?))
        }
    }

    pub async fn get_media(&self, media_id: i64) -> Result<Box<dyn MediaItem>> {
        let path = if self.radarr { "movie" } else { "episode" };

        let url = format!(
            "{}/{}/{}/{}?apikey={}",
            self.source_url,
            Self::API_PATH,
            path,
            media_id,
            self.api_key
        );

        let response = self
            .client
            .get(&url)
            .header(ACCEPT, "application/json")
            .send()
            .await?;

        ensure!(
            response.status().is_success(),
            "Failed to get media {}",
            media_id
        );

        if self.radarr {
            Ok(Box::new(response.json::<radarr::Movie>().await?))
        } else {
            Ok(Box::new(response.json::<sonarr::Episode>().await?))
        }
    }

    pub async fn delete_queue_record(&self, record: &Record) -> Result<()> {
        let url = format!(
            "{}/{}/queue/{}?{}&skipRedownload={}&apikey={}",
            self.source_url,
            Self::API_PATH,
            record.id,
            Self::DELETE_PARAMS,
            self.skip_redownload,
            self.api_key
        );

        let response = self
            .client
            .delete(&url)
            .header(ACCEPT, "*/*")
            .send()
            .await?;

        ensure!(
            response.status().is_success(),
            "Failed to delete record {}. Bad status code: {}",
            record.title,
            response.status()
        );

        Ok(())
    }

    pub async fn delete_episode_file(&self, record: &Record) -> Result<()> {
        let path = if self.radarr {
            "moviefile"
        } else {
            "episodefile"
        };

        let media = self.get_media(record.media_id).await?;

        let file_id = media
            .get_file_id()
            .ok_or(anyhow::anyhow!("Record does not have a file"))?;

        let url = format!(
            "{}/{}/{}/{}?apikey={}",
            self.source_url,
            Self::API_PATH,
            path,
            file_id,
            self.api_key,
        );

        let response = self
            .client
            .delete(&url)
            .header(ACCEPT, "*/*")
            .send()
            .await?;

        ensure!(
            response.status().is_success(),
            "Failed to delete file for record {}. Bad status code: {}",
            record.title,
            response.status()
        );

        Ok(())
    }
}

pub trait QueueJson {
    fn get_records(&self) -> Vec<Record>;
}

impl QueueJson for sonarr::Queue {
    fn get_records(&self) -> Vec<Record> {
        self.records.iter().map(Record::from).collect()
    }
}

impl QueueJson for radarr::Queue {
    fn get_records(&self) -> Vec<Record> {
        self.records.iter().map(Record::from).collect()
    }
}

pub trait MediaItem {
    fn get_file_id(&self) -> Option<i64>;
}

impl MediaItem for sonarr::Episode {
    fn get_file_id(&self) -> Option<i64> {
        self.episode_file_id
    }
}

impl MediaItem for radarr::Movie {
    fn get_file_id(&self) -> Option<i64> {
        self.movie_file_id
    }
}

#[derive(Debug)]
pub struct Record {
    id: i64,
    media_id: i64,
    title: String,
    tracked_status: String,
    status: String,
}

impl Record {
    pub fn get_tracked_status(&self) -> &str {
        &self.tracked_status
    }

    pub fn get_status(&self) -> &str {
        &self.status
    }
}

impl From<&sonarr::Record> for Record {
    fn from(record: &sonarr::Record) -> Self {
        Self {
            id: record.id,
            media_id: record.episode_id,
            title: record.title.clone(),
            tracked_status: record.tracked_download_status.clone(),
            status: record.status.clone(),
        }
    }
}

impl From<&radarr::Record> for Record {
    fn from(record: &radarr::Record) -> Self {
        Self {
            id: record.id,
            media_id: record.movie_id,
            title: record.title.clone(),
            tracked_status: record.tracked_download_status.clone(),
            status: record.status.clone(),
        }
    }
}

impl fmt::Display for Record {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.id, self.title)
    }
}
