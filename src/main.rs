mod api;
mod json;

use anyhow::Result;
use api::Api;
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

    /// Is this a Radarr instance
    #[arg(long, value_parser, default_value = "false")]
    radarr: bool,

    /// Skip attempting to redownload release
    #[arg(long, value_parser, default_value = "false")]
    skip_redownload: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let api = Api::new(cli.url, cli.api_key, cli.radarr, cli.skip_redownload);
    let records = api.get_queue().await?.get_records();

    // Search records for failed imports and downloads
    for record in records {
        if record.get_status() == "warning" {
            println!("Deleting record: {}", record);
            // Try to delete from queue
            if api.delete_queue_record(&record).await.is_err() {
                continue;
            };
            // Try to delete file (for files that imported)
            if api.delete_episode_file(&record).await.is_err() {
                continue;
            };
        }
    }

    Ok(())
}
