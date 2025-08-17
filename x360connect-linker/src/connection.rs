use futures::stream::{FuturesUnordered, StreamExt};
use if_addrs::get_if_addrs;
use ipnetwork::IpNetwork;
use std::net::{IpAddr, SocketAddr};
use std::vec;
use tokio::net::TcpStream;
use tokio::time::{Duration, timeout};
use crate::connection::verify_connection::verify_connection;

use crate::nova::utils::get_url;
use crate::nova::verify_connection;

fn mask_width<const SZ: usize>(octets: Option<[u8; SZ]>) -> u8 {
    octets
        .map(|bytes| bytes.iter().map(|&b| b.count_ones() as u8).sum())
        .unwrap_or(0)
}

fn prefix_len_v4(netmask: std::net::Ipv4Addr) -> u8 {
    mask_width(Some(netmask.octets()))
}

// fn prefix_len_v6(netmask: std::net::Ipv6Addr) -> u8 {
//     mask_width(Some(netmask.octets()))
// }

async fn get_networks() -> anyhow::Result<Vec<IpNetwork>> {
    let mut networks: Vec<IpNetwork> = vec![];
    for iface in get_if_addrs().expect("Falha ao listar interfaces") {
        let network: Option<IpNetwork> = match iface.addr {
            if_addrs::IfAddr::V4(ifv4_addr) => {
                let ip = ifv4_addr.ip;
                let prefix = prefix_len_v4(ifv4_addr.netmask);
                Some(IpNetwork::new(IpAddr::V4(ip), prefix)?)
            }
            // if_addrs::IfAddr::V6(ifv6_addr) => {
            //     let ip = ifv6_addr.ip;
            //     let prefix = prefix_len_v6(ifv6_addr.netmask);

            //     Some(IpNetwork::new(IpAddr::V6(ip), prefix)?)
            // }
            _ => {None}
        };
        if let Some(network) = network{
            networks.push(network);
        } 
    }
    Ok(networks)
}

pub async fn search_url(ports: Vec<u16>, lowest_allowed_prefix: Option<u8>) -> anyhow::Result<Option<String>> {
    let mut networks = get_networks().await?;
    networks.sort_by_key(|net| std::cmp::Reverse(net.prefix()));
    let lowest_allowed_prefix = match lowest_allowed_prefix {
        Some(p) => p,
        None => {
            24
        },
    };
    networks = networks.iter().filter(|ip| ip.prefix() >= lowest_allowed_prefix).cloned().collect();


    
    for network in networks {
        let mut futures = FuturesUnordered::new();
        
        for ip in network.iter() {
            for port in ports.clone() {
                
            let socket = SocketAddr::new(ip, port);
            futures.push(async move {
                match timeout(Duration::from_millis(500), TcpStream::connect(socket)).await {
                    Ok(Ok(_)) => Some((ip, port)),
                    _ => None,
                }
            });
            }
        }

        while let Some(result) = futures.next().await {
            match result {
                Some((ip, port)) => {
                    let url = get_url(ip, port);
                    let html_detected = verify_connection(url.clone()).await?;
                    if html_detected{
                        return Ok(Some(url));
                    }
                },
                None => {},
            }
        }

    }

    Ok(None)
}