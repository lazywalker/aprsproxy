use log::info;

mod dns;
mod filelog;
mod forwarder;
mod passcode;
mod relay;

#[tokio::main]
async fn main() {
    // return println!("{:?}", <aprsproxy::Opt as structopt::StructOpt>::from_args());
    init_log();

    info!("Starting up...");
    if aprsproxy::CONFIG.replace_from.len() != aprsproxy::CONFIG.replace_with.len() {
        panic!("replace-from and replace-with must be the same length");
    }

    relay::serv().await.unwrap();
}

/**
Initialize the logger
*/
fn init_log() {
    // use default filter from env RUST_LOG, if no filter is specified then use -q or -v
    let env = env_logger::Env::default().default_filter_or(
        match aprsproxy::CONFIG.quiet {
            true => log::LevelFilter::Off,
            false => match aprsproxy::CONFIG.verbose {
                0 => log::LevelFilter::Warn,
                1 => log::LevelFilter::Info,
                2 => log::LevelFilter::Debug,
                3..=9 | _ => log::LevelFilter::Trace,
            },
        }
        .as_str(),
    );

    use chrono::Local;
    use std::io::Write;
    env_logger::Builder::from_env(env)
        .format(|buf, record| {
            writeln!(
                buf,
                "{} {:5} {} - {}",
                Local::now().format("%Y-%m-%d %H:%M:%S%.3f%Z"),
                buf.default_styled_level(record.level()),
                record.module_path().unwrap_or("<unnamed>"),
                &record.args()
            )
        })
        .init();
}
