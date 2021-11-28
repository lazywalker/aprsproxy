use aprsproxy::CONFIG;
use log::{debug, error, info, trace, warn};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::tcp::{ReadHalf, WriteHalf};
use tokio::net::{TcpListener, TcpStream};

use futures::FutureExt;
use std::error::Error;

use crate::filelog;
use crate::{dns, forwarder};

pub async fn serv() -> Result<(), Box<dyn Error>> {
    filelog::init();
    let listen_addr = &CONFIG.local_addr;
    let proxy_addr = resolve_addr(CONFIG.remote_addr.as_str()).await;
    info!("Listening on: {}", listen_addr);
    info!("Proxying to: {}", proxy_addr);

    let listener = TcpListener::bind(listen_addr).await?;

    while let Ok((inbound, peer_addr)) = listener.accept().await {
        info!("A new connection {:?} is coming!", peer_addr);
        let transfer = transfer(inbound, proxy_addr.clone()).map(|r| {
            if let Err(e) = r {
                error!("Failed to transfer; error={}", e);
            }
        });

        tokio::spawn(transfer);
    }

    Ok(())
}

async fn transfer(mut inbound: TcpStream, proxy_addr: String) -> Result<(), Box<dyn Error>> {
    let mut outbound = TcpStream::connect(proxy_addr).await?;

    let (mut ri, mut wi) = inbound.split();
    let (mut ro, mut wo) = outbound.split();

    let client_to_server = async {
        copy_data_to_server(&mut ri, &mut wo).await?;
        wo.shutdown().await
    };

    let server_to_client = async {
        copy_data_to_client(&mut ro, &mut wi).await?;
        wi.shutdown().await
    };

    tokio::try_join!(client_to_server, server_to_client)?;

    Ok(())
}

async fn copy_data_to_server(
    reader: &mut ReadHalf<'_>,
    writer: &mut WriteHalf<'_>,
) -> Result<(), std::io::Error> {
    let mut buf = vec![0u8; 0x1024];
    let mut n: usize;

    loop {
        n = reader.read(&mut buf).await?;
        if n == 0 {
            break;
        }

        let mut line: String = String::from_utf8_lossy(&buf[..n]).to_string();
        // handle the replacement, if any
        if CONFIG.replace_from.is_empty() && CONFIG.replace_with.is_empty() {
            for (i, s) in CONFIG.replace_from.iter().enumerate() {
                line = line.replace(s.as_str(), CONFIG.replace_with[i].as_str());
            }
        }

        info!("{}", line.trim_end());
        filelog::log(line.as_str());

        // handle the forwarder
        let mut need_to_forward = false;
        let mut callsign = "";
        if CONFIG.forward_with.is_empty() {
            for (_, s) in CONFIG.forward_with.iter().enumerate() {
                if line.starts_with(s.as_str()) {
                    need_to_forward = true;
                    callsign = s.as_str();
                    break;
                }
            }

            trace!("need_to_forward = {}", need_to_forward);
            if need_to_forward {
                match forwarder::post(CONFIG.forward_to.as_str(), callsign, line.as_str()).await {
                    Ok(msg) => {
                        debug!("Forwarded: {}", msg.trim_end());
                    }
                    Err(e) => {
                        error!("Failed to forward: {}", e);
                    }
                }
            }
        }

        if !need_to_forward {
            writer.write_all(line.as_bytes()).await?;
        }
    }

    writer.flush().await?;
    Ok(())
}

async fn copy_data_to_client(
    reader: &mut ReadHalf<'_>,
    writer: &mut WriteHalf<'_>,
) -> Result<(), std::io::Error> {
    let mut buf = vec![0u8; 0x1024];
    let mut n: usize;

    loop {
        n = reader.read(&mut buf).await?;
        if n == 0 {
            break;
        }

        writer.write_all(&buf[..n]).await?;

        let line: String = String::from_utf8_lossy(&buf[..n]).to_string();

        if line.contains("Invalid") || line.contains("logresp") {
            warn!("{}", line.trim_end());
        } else {
            trace!("{}", line.trim_end());
        }
    }

    writer.flush().await?;
    Ok(())
}

async fn resolve_addr(addr_str: &str) -> String {
    let addr_parsed: Vec<&str> = addr_str.split(':').collect();
    let host = addr_parsed[0].to_string();

    match dns::resolve_single(host).await {
        Some(ip) => format!("{}:{}", ip.to_string(), addr_parsed[1]),
        None => addr_str.to_string(),
    }
}
