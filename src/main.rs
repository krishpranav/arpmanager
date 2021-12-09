extern crate pnet;
extern crate clap;
pub mod cli;
pub mod configure;
use std::thread;
use std::time::Duration;
use std::net::Ipv4Addr;
use pnet::datalink::{self, NetworkInterface};
use pnet::datalink::Channel;
use pnet::packet::ethernet::MutableEthernetPacket;
use pnet::packet::arp::MutableArpPacket;
use pnet::util::MacAddr;
use pnet::packet::ethernet::EtherTypes;
use pnet::packet::MutablePacket;
use pnet::packet::arp::{ArpHardwareTypes, ArpOperations, ArpOperation};
use configure::Configure;
use cli::cli_main;


fn send_arp_packet(interface: NetworkInterface, source_ip: Ipv4Addr, source_mac: MacAddr, target_ip: Ipv4Addr, target_mac: MacAddr, arp_operation: ArpOperation) {
    let(mut tx, _) = match datalink::channel(&interface, Default::default()) {
        Ok(Channel::Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unknown channel type"),
        Err(e) => panic!("Error happened {}", e),
    };

    let mut ethernet_buffer = [0u8; 42];
    let mut ethernet_packet = MutableEthernetPacket::new(&mut ethernet_buffer).unwrap();

    ethernet_packet.set_destination(target_mac);
    ethernet_packet.set_source(source_mac);
    ethernet_packet.set_ethertype(EtherTypes::Arp);

    let mut arp_buffer = [0u8; 28];
    let mut arp_packet = MutableArpPacket::new(&mut arp_buffer).unwrap();

    arp_packet.set_hardware_type(ArpHardwareTypes::Ethernet);
    arp_packet.set_protocol_type(EtherTypes::Ipv4);
    arp_packet.set_hw_addr_len(6);
    arp_packet.set_proto_addr_len(4);
    arp_packet.set_operation(arp_operation);
    arp_packet.set_sender_hw_addr(source_mac);
    arp_packet.set_sender_proto_addr(source_ip);
    arp_packet.set_target_hw_addr(target_mac);
    arp_packet.set_target_proto_addr(target_ip);

    ethernet_packet.set_payload(arp_packet.packet_mut());

    tx.send_to(&ethernet_packet.to_immutable(), Some(interface));
}

fn main() {
    let configure: Configure = cli_main();
    let mut packet_count: i32 = 0;

    loop {
        let interfaces = datalink::interfaces();

        let interfaces_name_match = |iface: &NetworkInterface| iface.name == configure.interface;
        let interface = interfaces.into_iter().filter(interfaces_name_match).next().unwrap();
       
        send_arp_packet(interface, configure.source_ip, configure.source_mac, configure.target_ip, configure.target_mac, configure.arp_operation);

        packet_count += 1;
        let arp_operation = match configure.arp_operation {
            ArpOperations::Request => "Request",
            ArpOperations::Reply => "Reply",
            ArpOperation(_) => panic!("Unknown operation")
        };
        println!("Sent {} ARP {} packets.", packet_count, arp_operation);
        thread::sleep(Duration::new(5, 0));
    }
}