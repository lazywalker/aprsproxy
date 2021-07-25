use std::{net::AddrParseError, str::FromStr};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "AprsProxy",
    about = r"
   ___                ___                   
  / _ | ___  _______ / _ \_______ __ ____ __
 / __ |/ _ \/ __(_-</ ___/ __/ _ \\ \ / // /
/_/ |_/ .__/_/ /___/_/  /_/  \___/_\_\\_, / 
     /_/                             /___/  
                A simply APRS-IS proxy tool.
"
)]
pub struct Opt {
    /// The local address and port to listen on
    #[structopt(short = "l", long = "local", parse(try_from_str = parse_ipaddr), default_value = "0.0.0.0:14580")]
    pub local_addr: String,

    /// The remote address and port to connect to
    #[structopt(short = "r", long = "remote", default_value = "china.aprs2.net:14580")]
    pub host_addr: String,

    /// The text to be replaced
    #[structopt(long = "replace")]
    pub replace_from: Option<String>,

    /// The text to replace with
    #[structopt(long = "with")]
    pub replace_with: Option<String>,
}

fn parse_ipaddr(addr_str: &str) -> Result<String, AddrParseError> {
    let addr_parsed: Vec<&str> = addr_str.split(":").collect();
    if addr_parsed.len() != 2 {
        panic!("address is incorrect!");
    }
    let addr = addr_parsed[0].to_string();
    let port = u16::from_str(addr_parsed[1]).expect("invaild port");
    Ok(format!("{}:{}", addr, port))
}

#[derive(Clone)]
pub struct ProxyConfig {
    pub local_addr: String,
    pub remote_addr: String,
    pub replace_from: Option<String>,
    pub replace_with: Option<String>,
}

impl ProxyConfig {
    pub fn parse() -> ProxyConfig {
        let opt = Opt::from_args();

        ProxyConfig {
            local_addr: opt.local_addr,
            remote_addr: opt.host_addr,
            replace_from: opt.replace_from,
            replace_with: opt.replace_with,
        }
    }
}
