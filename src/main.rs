use log::info;

mod dns;
mod relay;
mod filelog;

#[tokio::main]
async fn main() {
    env_logger::init();
    info!("Starting up...");
    relay::serv().await.unwrap();
}
