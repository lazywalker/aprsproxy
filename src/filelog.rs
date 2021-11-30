use chrono::Local;

use std::{
    fs::{self, OpenOptions},
    io::Write,
};

use aprsproxy::CONFIG;

pub fn init() {
    if CONFIG.filelog {
        fs::create_dir_all("./log").expect("Can not create log dir!");
    }
}

pub fn log(msg: &str) -> Result<(), std::io::Error> {
    if !CONFIG.filelog {
        return Ok(());
    }

    let now = Local::now();
    // rotate log file
    let filename = format!("./log/{}.log", now.format("%Y%m%d"));
    // appending time to fornt of every line
    let logtext = format!("{} {}", now.format("%Y-%m-%dT%H:%M:%S%.3f%Z"), msg);

    let mut f = OpenOptions::new()
        .append(true)
        .create(true)
        .open(filename)?;

    f.write_all(logtext.as_bytes())
}
