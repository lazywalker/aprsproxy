use aprsproxy::ProxyConfig;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::tcp::{ReadHalf, WriteHalf};
use tokio::net::{TcpListener, TcpStream};

use futures::FutureExt;
use std::error::Error;

use crate::dns;

pub async fn serv(conf: ProxyConfig) -> Result<(), Box<dyn Error>> {
    let listen_addr = conf.local_addr;
    let proxy_addr = resolve_addr(conf.remote_addr).await;
    println!("Listening on: {}", listen_addr);
    println!("Proxying to: {}", proxy_addr);

    let listener = TcpListener::bind(listen_addr).await?;

    while let Ok((inbound, _)) = listener.accept().await {
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

        let line = String::from_utf8_lossy(&buf[..n]);
        print!("{}", line);
        writer.write_all(&buf[..n]).await?;
    }
    io::stdout().flush().await?;
    writer.flush().await?;
    Ok(())
}

async fn resolve_addr(addr_str: String) -> String {
    let addr_parsed: Vec<&str> = addr_str.split(":").collect();
    let host = addr_parsed[0].to_string();

    match dns::resolve_single(host).await {
        Some(ip) => format!("{}:{}", ip.to_string(), addr_parsed[1]),
        None => addr_str,
    }
}
