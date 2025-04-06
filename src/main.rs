use std::sync::mpsc::channel;

use types::GetContentMessage;

mod content_manager;
mod file_system;
mod server;
mod templating;
mod types;
mod web_content;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let (tx, rx) = channel::<GetContentMessage>();

    content_manager::start(rx);

    server::start(tx).await
}
