mod json;

use anyhow::{ensure, Result};
use clap::Parser;
use json::SonarrQueue;
use reqwest::{header::ACCEPT, Client, Response};

/// Automatically delete and request failed downloads and imports from Sonarr. (Radarr coming soon)
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Url for Sonarr instance -- Ex: http://localhost:8989
    #[arg(value_parser)]
    url: String,

    /// Sonarr API key
    #[arg(value_parser)]
    api_key: String,

    /// Skip attempting to redownload release
    #[arg(long, value_parser, default_value = "false")]
    skip_redownload: bool,
}

async fn delete_from_queue(
    client: &Client,
    source_url: &str,
    id: i64,
    skip_redownload: bool,
    api_key: &str,
) -> Result<Response, reqwest::Error> {
    let delete_url = format!("{}/api/v3/queue/{}?removeFromClient=false&blocklist=true&skipRedownload={}&changeCategory=false&apikey={}", source_url, id, skip_redownload, api_key);
    client
        .delete(delete_url)
        .header(ACCEPT, "*/*")
        .send()
        .await?
        .error_for_status()
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let source_url = cli.url;
    let api_key = cli.api_key;

    let client = Client::new();
    let queue_url = format!("{}/api/v3/queue?page=1&pageSize=1000&includeUnknownSeriesItems=false&includeSeries=false&includeEpisode=false&apikey={}", source_url, api_key);

    let response = client
        .get(&queue_url)
        .header(ACCEPT, "application/json")
        .send()
        .await?;
    let queue: json::SonarrQueue = response.json().await?;

    // Search records for failed imports and downloads
    for record in queue.records {
        if record.tracked_download_status == "warning" {
            println!(
                "{}    Status: {}    Deleting...",
                record.tracked_download_state, record.title
            );
            let _ = delete_from_queue(
                &client,
                &source_url,
                record.id,
                cli.skip_redownload,
                &api_key,
            )
            .await?;
        }
    }

    Ok(())
}
