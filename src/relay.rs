use aprsproxy::ProxyConfig;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::tcp::{ReadHalf, WriteHalf};
use tokio::net::{TcpListener, TcpStream};

use futures::FutureExt;
use lazy_static::lazy_static;
use std::error::Error;

use crate::dns;

lazy_static! {
    static ref CONF: ProxyConfig = ProxyConfig::parse();
}

pub async fn serv() -> Result<(), Box<dyn Error>> {
    let listen_addr = &CONF.local_addr;
    let proxy_addr = resolve_addr(CONF.remote_addr.as_str()).await;
    println!("Listening on: {}", listen_addr);
    println!("Proxying to: {}", proxy_addr);

    let listener = TcpListener::bind(listen_addr).await?;

    while let Ok((inbound, peer_addr)) = listener.accept().await {
        println!("A new connection {:?} is coming!", peer_addr);
        let transfer = transfer(inbound, proxy_addr.clone()).map(|r| {
            if let Err(e) = r {
                println!("Failed to transfer; error={}", e);
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
        io::copy(&mut ro, &mut wi).await?;
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
        if CONF.replace_from.is_some() && CONF.replace_with.is_some() {
            line = line.replace(
                CONF.replace_from.as_ref().unwrap(),
                CONF.replace_with.as_ref().unwrap(),
            );

            writer.write_all(&line.as_bytes()).await?;
        } else {
            writer.write_all(&buf[..n]).await?;
        }
        print!("{}", line);
    }
    io::stdout().flush().await?;
    writer.flush().await?;
    Ok(())
}

async fn resolve_addr(addr_str: &str) -> String {
    let addr_parsed: Vec<&str> = addr_str.split(":").collect();
    let host = addr_parsed[0].to_string();

    match dns::resolve_single(host).await {
        Some(ip) => format!("{}:{}", ip.to_string(), addr_parsed[1]),
        None => addr_str.to_string(),
    }
}
