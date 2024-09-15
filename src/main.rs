mod api;
mod json;

use anyhow::Result;
use api::{Api, Record};
use clap::Parser;

/// Automatically delete and request failed downloads and imports from Sonarr/Radarr
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Url for Sonarr/Radarr instance -- Ex: http://localhost:8989
    #[arg(value_parser)]
    url: String,

    /// Sonarr/Radarr API key
    #[arg(value_parser)]
    api_key: String,

    /// Use Radarr parsing instead of Sonarr
    #[arg(long, value_parser, default_value = "false")]
    radarr: bool,

    /// Skip attempting to redownload release
    #[arg(long, value_parser, default_value = "false")]
    skip_redownload: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    const BAD_STATUS: &str = "warning";
    let cli = Cli::parse();
    let api = Api::new(cli.url, cli.api_key, cli.radarr, cli.skip_redownload);
    let records = api.get_queue().await?.get_records();

    // Filter out successfull records
    let records: Vec<Record> = records
        .into_iter()
        .filter(|record| {
            record.get_tracked_status() == BAD_STATUS || record.get_status() == BAD_STATUS
        })
        .collect();

    // Delete failed records from files and queue
    println!("Trying to delete {} records", records.len());
    for record in &records {
        if let Err(e) = api.delete_episode_file(record).await {
            println!("Failed to delete: {:?}", e);
            continue;
        };
    }

    for record in &records {
        if api.delete_queue_record(record).await.is_err() {
            continue;
        };
    }

    Ok(())
}
