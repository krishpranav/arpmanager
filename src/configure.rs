use std::net::Ipv4Addr;
use pnet::util::MacAddr;
use pnet::packet::arp::ArpOperation;

#[derive(Debug)]
pub struct Configure {
    pub interface: String,
    pub source_ip: Ipv4Addr,
    pub source_mac: MacAddr,
    pub target_ip: Ipv4Addr,
    pub target_mac: MacAddr,
    pub arp_operation: ArpOperation
}