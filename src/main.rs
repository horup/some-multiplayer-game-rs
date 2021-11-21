use hostess::{log::LevelFilter, server::Server, tokio, game_server::{GameServerConstructor, GameServer}, uuid::Uuid};
mod server;
mod bot;

#[tokio::main]
async fn main() {
    env_logger::builder().filter_level(LevelFilter::Info).init();

    let f = || {
        let boxed:Box<dyn GameServer> = Box::new(server::Server::new());
        return boxed;
    };
    let constructor = GameServerConstructor::new(Box::new(f));
    let server:Server = Server::new("0.0.0.0:8080", constructor.clone());
    server.lobby.write().await.new_host(Uuid::nil(), constructor);
    let _ = server.spawn().await; 
}