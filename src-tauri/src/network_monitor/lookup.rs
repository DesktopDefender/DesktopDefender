use hickory_resolver::config::*;
use hickory_resolver::Resolver;
use std::net::IpAddr;
use std::net::Ipv4Addr;

use crate::HOSTNAME_CACHE;

pub fn add_ip_to_cache(ip: Ipv4Addr) -> Result<String, String> {
    let mut cache = HOSTNAME_CACHE.lock();
    if let Some(name) = cache.get(&ip) {
        return Ok(name.to_string());
    }

    let name = reverse_dns_lookup(ip)?;
    cache.insert(ip, name.clone());
    Ok(name)
}

fn reverse_dns_lookup(ip: Ipv4Addr) -> Result<String, String> {
    let resolver = Resolver::new(ResolverConfig::default(), ResolverOpts::default())
        .map_err(|e| e.to_string())?;

    if ip.is_private() {
        return Err("IP is private".into());
    }

    let response = resolver
        .reverse_lookup(IpAddr::V4(ip))
        .map_err(|e| e.to_string())?;
    match response.iter().next() {
        Some(name) => Ok(name.to_utf8()),
        None => Err("No name found for IP".into()),
    }
}
