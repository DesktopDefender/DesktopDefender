// use hickory_resolver::config::*;
// use hickory_resolver::Resolver;
// use std::net::IpAddr;
use std::net::Ipv4Addr;

use crate::IP_CACHE;
use crate::IP_SET;

pub async fn add_ip_to_set(ip: Ipv4Addr) -> Result<String, String> {
    let mut ip_set = IP_SET.lock().await;
    let cache = IP_CACHE.lock().await;
    if ip_set.contains(&ip) || cache.contains_key(&ip.to_string()) || ip.is_private() {
        Err("IP already in set".to_string())
    } else {
        ip_set.insert(ip);
        Ok(ip.to_string())
    }
}

// fn reverse_dns_lookup(ip: Ipv4Addr) -> Result<String, String> {
//     let resolver = Resolver::new(ResolverConfig::default(), ResolverOpts::default())
//         .map_err(|e| e.to_string())?;
//
//     if ip.is_private() {
//         return Err("IP is private".into());
//     }
//
//     let response = resolver
//         .reverse_lookup(IpAddr::V4(ip))
//         .map_err(|e| e.to_string())?;
//     match response.iter().next() {
//         Some(name) => Ok(name.to_utf8()),
//         None => Err("No name found for IP".into()),
//     }
// }
