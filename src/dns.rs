use log::{error, info};
use std::net;
use trust_dns_resolver::{
    config::{ResolverConfig, ResolverOpts},
    TokioAsyncResolver,
};

pub async fn resolve_single(addr: String) -> Option<net::IpAddr> {
    let ip = addr.parse::<net::IpAddr>();
    if ip.is_ok() {
        return Some(ip.unwrap());
    }

    let resolver =
        async { TokioAsyncResolver::tokio(ResolverConfig::default(), ResolverOpts::default()) }
            .await
            .unwrap();
    let remote_addr = format!("{}.", addr);

    info!("Resolving ip address...");
    let res = resolver.lookup_ip(remote_addr).await.unwrap();

    match res.iter().find(|ip| ip.is_ipv4()) {
        Some(ip_v4) => Some(ip_v4),
        None => {
            if let Some(ip_v6) = res.iter().find(|ip| ip.is_ipv6()) {
                Some(ip_v6)
            } else {
                error!("Cannot resolve {}", addr);
                return None;
            }
        }
    }
}
