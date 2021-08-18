use log::info;

mod dns;
mod filelog;
mod forwarder;
mod passcode;
mod relay;

#[tokio::main]
async fn main() {
    // return println!("{:?}", <aprsproxy::Opt as structopt::StructOpt>::from_args());
    aprsproxy::init_logger();

    info!("Starting up...");
    if aprsproxy::CONFIG.replace_from.len() != aprsproxy::CONFIG.replace_with.len() {
        panic!("replace-from and replace-with must be the same length");
    }

    relay::serv().await.unwrap();
}
