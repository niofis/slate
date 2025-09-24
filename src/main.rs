use std::sync::mpsc::channel;

use types::GetContentMessage;

mod content_manager;
mod file_system;
mod server;
mod stats;
mod templating;
mod types;
mod web_content;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    // Initialize statistics database
    let stats_manager = stats::StatsManager::new("./blog_stats.db")
        .expect("Failed to initialize statistics database");

    let (tx, rx) = channel::<GetContentMessage>();

    content_manager::start(rx);

    server::start(tx, stats_manager).await
}
