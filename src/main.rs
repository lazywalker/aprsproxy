mod dns;
mod relay;
mod filelog;

#[tokio::main]
async fn main() {
    relay::serv().await.unwrap();
}
