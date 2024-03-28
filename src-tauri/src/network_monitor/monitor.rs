// extern crate pcap;
// extern crate pnet;

use colored::*;
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::{
    ethernet::{EtherTypes, EthernetPacket},
    ipv4::Ipv4Packet,
    tcp::TcpPacket,
    udp::UdpPacket,
    Packet,
};

#[tauri::command]
pub fn listen_to_traffic() {
    let devices = match pcap::Device::list() {
        Ok(devices) => devices,
        Err(e) => {
            println!("Error retrieving device list: {}", e);
            return;
        }
    };

    if devices.is_empty() {
        println!("No available network devices found.");
        return;
    }

    let mut cap = match pcap::Capture::from_device("en0") {
        Ok(dev) => match dev.promisc(true).open() {
            Ok(cap) => cap,
            Err(e) => {
                println!("Error opening capture on device: {}", e);
                return;
            }
        },
        Err(e) => {
            println!("Error finding device 'en0': {}", e);
            return;
        }
    };

    while let Ok(packet) = cap.next_packet() {
        process_packet(&packet);
    }
}

fn process_packet(packet: &pcap::Packet) {
    if let Some(ethernet_packet) = EthernetPacket::new(&packet.data) {
        match ethernet_packet.get_ethertype() {
            EtherTypes::Ipv4 => {
                if let Some(ipv4_packet) = Ipv4Packet::new(ethernet_packet.payload()) {
                    match ipv4_packet.get_next_level_protocol() {
                        IpNextHeaderProtocols::Tcp => {
                            process_tcp_packet(&ipv4_packet, packet.header.len)
                        }
                        IpNextHeaderProtocols::Udp => {
                            process_udp_packet(&ipv4_packet, packet.header.len)
                        }
                        _ => (),
                    }
                }
            }
            _ => (),
        }
    }
}

fn process_tcp_packet(ipv4_packet: &Ipv4Packet, original_packet_length: u32) {
    if let Some(tcp_packet) = TcpPacket::new(ipv4_packet.payload()) {
        println!(
            "{}",
            format!(
                "TCP Packet: {}:{} -> {}:{}; Len: {}",
                ipv4_packet.get_source(),
                tcp_packet.get_source(),
                ipv4_packet.get_destination(),
                tcp_packet.get_destination(),
                original_packet_length,
            )
            .bright_blue()
        );
    }
}

fn process_udp_packet(ipv4_packet: &Ipv4Packet, original_packet_length: u32) {
    if let Some(udp_packet) = UdpPacket::new(ipv4_packet.payload()) {
        println!(
            "{}",
            format!(
                "UDP Packet: {}:{} -> {}:{}; Len: {}",
                ipv4_packet.get_source(),
                udp_packet.get_source(),
                ipv4_packet.get_destination(),
                udp_packet.get_destination(),
                original_packet_length,
            )
            .magenta()
        );
    }
}
