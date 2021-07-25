mod dns;
mod relay;

#[tokio::main]
async fn main() {
    relay::serv().await.unwrap();
}
