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

use config::Config;
use cli::cli_main;

fn send_arp_packet(interface: NetworkInterface, source_ip: Ipv4Addr, source_mac: MacAddr, target_ip: Ipv4Addr, target_mac: MacAddr, arp_operation: ArpOperation) { }
}