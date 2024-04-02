use super::lookup::add_ip_to_set;
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::{
    ethernet::{EtherTypes, EthernetPacket},
    ipv4::Ipv4Packet,
    tcp::TcpPacket,
    udp::UdpPacket,
    Packet,
};
use serde::{Deserialize, Serialize};
use serde_json;
use std::{thread, time::Duration};
use tauri::Window;

#[derive(Serialize, Deserialize)]
pub struct PacketInfo {
    protocol: String,
    source: String,
    destination: String,
    length: u32,
}

#[tauri::command]
pub fn init_traffic_emitter(window: Window) {
    tauri::async_runtime::spawn(async move {
        loop {
            match listen_to_traffic().await {
                Ok(packets_json) => {
                    window
                        .emit("packets", &packets_json)
                        .expect("Failed to emit event");
                }
                Err(e) => eprintln!("Error listening to traffic: {}", e),
            }
            thread::sleep(Duration::from_millis(100));
        }
    });
}

async fn listen_to_traffic() -> Result<String, String> {
    let devices = pcap::Device::list().map_err(|e| e.to_string())?;

    if devices.is_empty() {
        return Err("No available network devices found.".into());
    }

    let device = devices
        .into_iter()
        .find(|d| d.name == "en0")
        .ok_or("Device 'en0' not found.")?;

    let mut cap = pcap::Capture::from_device(device)
        .map_err(|e| e.to_string())?
        .promisc(true)
        .open()
        .map_err(|e| e.to_string())?
        .setnonblock()
        .map_err(|e| e.to_string())?;

    let mut packets = Vec::new();

    let start_time = std::time::Instant::now();
    let capture_duration = Duration::from_millis(1000);

    while start_time.elapsed() < capture_duration {
        if let Ok(packet) = cap.next_packet() {
            if let Some(packet_info) = process_packet(&packet).await {
                packets.push(packet_info);
            }
        }
    }
    serde_json::to_string(&packets).map_err(|e| e.to_string())
}

async fn process_packet(packet: &pcap::Packet<'_>) -> Option<PacketInfo> {
    let ethernet_packet = EthernetPacket::new(&packet.data)?;

    let (protocol, src, dest, length) = match ethernet_packet.get_ethertype() {
        EtherTypes::Ipv4 => {
            let ipv4_packet = Ipv4Packet::new(ethernet_packet.payload())?;
            match ipv4_packet.get_next_level_protocol() {
                IpNextHeaderProtocols::Tcp => {
                    let _tcp_packet = TcpPacket::new(ipv4_packet.payload())?;
                    (
                        "TCP".to_string(),
                        ipv4_packet.get_source(),
                        ipv4_packet.get_destination(),
                        packet.header.len,
                    )
                }
                IpNextHeaderProtocols::Udp => {
                    let _udp_packet = UdpPacket::new(ipv4_packet.payload())?;
                    (
                        "UDP".to_string(),
                        ipv4_packet.get_source(),
                        ipv4_packet.get_destination(),
                        packet.header.len,
                    )
                }
                _ => return None,
            }
        }
        _ => return None,
    };

    let source = add_ip_to_set(src).await.unwrap_or(src.to_string());
    let destination = add_ip_to_set(dest).await.unwrap_or(dest.to_string());

    Some(PacketInfo {
        protocol,
        source,
        destination,
        length,
    })
}
