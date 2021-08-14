use chrono::{
    format::{DelayedFormat, StrftimeItems},
    prelude::*,
};

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

pub fn log(msg: &str) {
    if !CONFIG.filelog {
        return;
    }
    
    let datefmt = "%Y%m%d";
    let dtfmt = "%Y-%m-%d %H:%M:%S";

    let now: DateTime<Local> = Local::now();
    let date: DelayedFormat<StrftimeItems> = now.format(datefmt);
    let dt: DelayedFormat<StrftimeItems> = now.format(dtfmt);
    let str_date: String = date.to_string(); // 20210727

    // rotate log file
    let filename = format!("./log/{}.log", str_date);
    // appending time to fornt of every line
    let logtext = format!("{} {}", dt.to_string(), msg);

    let mut f = OpenOptions::new()
        .append(true)
        .create(true)
        .open(filename)
        .unwrap();

    f.write(logtext.as_bytes()).unwrap();
}
