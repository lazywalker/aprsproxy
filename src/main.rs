use log::info;

mod dns;
mod filelog;
mod forwarder;
mod passcode;
mod relay;

#[tokio::main]
async fn main() {
    // return println!("{:?}", <aprsproxy::Opt as structopt::StructOpt>::from_args());
    // return println!("{:?}", std::env::var("RUST_LOG"));
    // init logger
    match std::env::var("RUST_LOG") {
        Ok(_) => {
            env_logger::init();
        }
        Err(_) => {
            let mut eb = env_logger::Builder::new();
            if aprsproxy::CONFIG.quiet {
                eb.filter(None, log::LevelFilter::Off);
            } else {
                match aprsproxy::CONFIG.verbose {
                    0 => eb.filter(None, log::LevelFilter::Warn),
                    1 => eb.filter(None, log::LevelFilter::Info),
                    2 => eb.filter(None, log::LevelFilter::Debug),
                    3..=9 | _ => eb.filter(None, log::LevelFilter::Trace),
                };
            }
            eb.init();
        }
    }

    info!("Starting up...");
    if aprsproxy::CONFIG.replace_from.len() != aprsproxy::CONFIG.replace_with.len() {
        panic!("replace-from and replace-with must be the same length");
    }

    relay::serv().await.unwrap();
}
