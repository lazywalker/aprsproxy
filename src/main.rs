mod dns;
mod relay;
use aprsproxy::ProxyConfig;

#[tokio::main]
async fn main() {
    let conf = ProxyConfig::parse();
    relay::serv(conf).await.unwrap();
}
