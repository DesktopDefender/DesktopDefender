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

#[derive(Serialize, Deserialize)]
pub struct PacketInfo {
    protocol: String,
    source: String,
    destination: String,
    length: u32,
}

#[tauri::command]
pub fn listen_to_traffic() -> Result<String, String> {
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
    let capture_duration = Duration::from_millis(250); // Adjust as needed

    while start_time.elapsed() < capture_duration {
        if let Ok(packet) = cap.next_packet() {
            if let Some(packet_info) = process_packet(&packet) {
                packets.push(packet_info);
            }
        }
        thread::sleep(Duration::from_millis(50));

        // for _ in 0..10000{
        //     if let Ok(packet) = cap.next_packet() {
        //         if let Some(packet_info) = process_packet(&packet) {
        //             packets.push(packet_info);
        //         }
        //     } else {
        //         continue;
        //     }
        // }
    }
    serde_json::to_string(&packets).map_err(|e| e.to_string())
}

fn process_packet(packet: &pcap::Packet) -> Option<PacketInfo> {
    let ethernet_packet = EthernetPacket::new(&packet.data)?;

    let (protocol, source, destination, length) = match ethernet_packet.get_ethertype() {
        EtherTypes::Ipv4 => {
            let ipv4_packet = Ipv4Packet::new(ethernet_packet.payload())?;
            match ipv4_packet.get_next_level_protocol() {
                IpNextHeaderProtocols::Tcp => {
                    let _tcp_packet = TcpPacket::new(ipv4_packet.payload())?;
                    (
                        "TCP".to_string(),
                        ipv4_packet.get_source().to_string(),
                        ipv4_packet.get_destination().to_string(),
                        packet.header.len,
                    )
                }
                IpNextHeaderProtocols::Udp => {
                    let _udp_packet = UdpPacket::new(ipv4_packet.payload())?;
                    (
                        "UDP".to_string(),
                        ipv4_packet.get_source().to_string(),
                        ipv4_packet.get_destination().to_string(),
                        packet.header.len,
                    )
                }
                _ => return None,
            }
        }
        _ => return None,
    };

    Some(PacketInfo {
        protocol,
        source,
        destination,
        length,
    })
}
