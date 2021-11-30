use std::error::Error;

use log::info;

mod dns;
mod filelog;
mod forwarder;
mod passcode;
mod relay;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // return println!("{:?}", <aprsproxy::Opt as structopt::StructOpt>::from_args());
    aprsproxy::init_logger();

    info!("Starting up...");
    if aprsproxy::CONFIG.replace_from.len() != aprsproxy::CONFIG.replace_with.len() {
        panic!("replace-from and replace-with must be the same length");
    }

    relay::serv().await
}
