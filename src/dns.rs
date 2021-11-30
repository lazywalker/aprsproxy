use log::{error, info};
use std::{
    error::Error,
    net::{self, AddrParseError},
};
use trust_dns_resolver::{
    config::{ResolverConfig, ResolverOpts},
    error::ResolveError,
    TokioAsyncResolver,
};

pub async fn resolve_single<E: Error + ?Sized>(addr: String) -> Result<net::IpAddr, Box<E>>
where
    Box<E>: From<AddrParseError>,
    Box<E>: From<ResolveError>,
{
    let ip = addr.parse::<net::IpAddr>();
    if ip.is_ok() {
        return Ok(ip?);
    }

    let resolver =
        async { TokioAsyncResolver::tokio(ResolverConfig::default(), ResolverOpts::default()) }
            .await?;
    let remote_addr = format!("{}.", addr);

    info!("Resolving ip address...");
    let res = resolver.lookup_ip(remote_addr).await?;

    match res.iter().find(|ip| ip.is_ipv4()) {
        Some(ip_v4) => Ok(ip_v4),
        None => {
            if let Some(ip_v6) = res.iter().find(|ip| ip.is_ipv6()) {
                Ok(ip_v6)
            } else {
                let err = format!("Cannot resolve {}", addr);
                error!("{}", err);
                Err(Box::from(ResolveError::from(err)))
            }
        }
    }
}
