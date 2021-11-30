use hostess::{master::{Master}, log::{LevelFilter, info}, server::Constructor, client::Uuid, tokio};

use crate::server::Server;
mod server;
mod bot;

#[tokio::main]
async fn main() {
  
    env_logger::builder().filter_level(LevelFilter::Info).init();

    let working_directory = std::env::current_dir().unwrap_or_default();
    info!("Working directory: {}", working_directory.to_str().unwrap_or_default());

    let mut server = Master::new("0.0.0.0:8080", Constructor::new::<Server>());
    
    for i in 0..8 {
        server.new_server(Uuid::nil()).await;
    }
    let _ = server.start().await; 
}