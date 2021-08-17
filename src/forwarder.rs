use reqwest::header::HeaderMap;

use crate::passcode;

pub async fn post(url: &str, callsign: &str, package: &str) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();
    let mut headers = HeaderMap::new();

    // Send-only specification see http://www.aprs-is.net/SendOnlyPorts.aspx
    headers.insert("Accept-Type", "text/plain".parse().unwrap());
    headers.insert("Content-Type", "application/octet-stream".parse().unwrap());

    let login_message = {
        let passcode = passcode::generate(callsign);
        let name = option_env!("CARGO_PKG_NAME").unwrap_or("unknown");
        let version = option_env!("CARGO_PKG_VERSION").unwrap_or("0.0.0");

        format!(
            "user {} pass {} vers {} {}",
            callsign, passcode, name, version
        )
    };

    let body = format!("{}\n{}", login_message, package);

    Ok(client
        .post(format!("http://{}", url))
        .headers(headers)
        .body(body)
        .send()
        .await?
        .text()
        .await?)
}

#[cfg(test)]
mod tests {
    use std::time::{SystemTime, UNIX_EPOCH};

    use super::*;

    macro_rules! aw {
        ($e:expr) => {
            tokio_test::block_on($e)
        };
    }

    #[test]
    fn test_post() {
        let url = "china.aprs2.net:8080";
        let callsign = "T3ST-13";
        let package = format!(
            "{}>APRS,TCPIP*,qAC,T2NANJING:>This is a test packet sn {}",
            callsign,
            timestamp()
        );
        assert_eq!(
            "ok".to_string(),
            aw!(post(url, callsign, package.as_str())).unwrap().trim_end()
        );
    }

    fn timestamp() -> i64 {
        let start = SystemTime::now();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        since_the_epoch.as_secs() as i64 * 1000i64
            + (since_the_epoch.subsec_nanos() as f64 / 1_000_000.0) as i64
    }
}
