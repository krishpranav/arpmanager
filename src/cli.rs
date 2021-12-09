use std::net::{Ipv4Addr, AddrParseError};
use pnet::util::{MacAddr, ParseMacAddrErr};
use pnet::packet::arp::{ArpOperation, ArpOperations};
use clap::{Arg, App};

use crate::configure::Configure;

const ABOUT: &'static str = "
Send an arp packet";

pub fn cli_main() -> Configure {
    let matches = App::new("arpmanager")
                          .version("0.1.0")
                          .author("Krisna Pranav")
                          .about(ABOUT)
                          .arg(Arg::with_name("interface")
                              .short("i")
                              .long("interface")
                              .required(true)
                              .takes_value(true)
                              .help("Provide the interface to be used to send packets"))
                          .arg(Arg::with_name("arp_reply")
                              .long("reply")
                              .help("Send an arp reply packet"))
                          .arg(Arg::with_name("source_ip")
                              .long("source-ip")
                              .required(true)
                              .takes_value(true)
                              .help("Set the source ip"))
                          .arg(Arg::with_name("source_mac")
                              .long("source-mac")
                              .required(true)
                              .takes_value(true)
                              .help("Set the source mac address"))
                          .arg(Arg::with_name("target_ip")
                              .long("target-ip")
                              .required(true)
                              .takes_value(true)
                              .help("Set the target ip"))
                          .arg(Arg::with_name("target_mac")
                              .long("target-mac")
                              .required(true)
                              .takes_value(true)
                              .help("Set the target mac address"))
                          .get_matches();

    let interface: String = matches.value_of("interface").unwrap().trim().to_string();

    let source_ip: Result<Ipv4Addr, AddrParseError> = matches.value_of("source_ip").unwrap()
                                                      .trim().parse();
    let source_mac: Result<MacAddr, ParseMacAddrErr> = matches.value_of("source_mac").unwrap()
                                                       .trim().parse();
    let target_ip: Result<Ipv4Addr, AddrParseError> = matches.value_of("target_ip").unwrap()
                                                      .trim().parse();
    let target_mac: Result<MacAddr, ParseMacAddrErr> = matches.value_of("target_mac").unwrap()
                                                       .trim().parse();

    let mut arp_operation: ArpOperation = ArpOperations::Request;

    if matches.is_present("arp_reply") {
        arp_operation = ArpOperations::Reply;
    }

    let configure: Configure = Configure {
        interface: interface,
        source_ip: source_ip.unwrap(),
        source_mac: source_mac.unwrap(),
        target_ip: target_ip.unwrap(),
        target_mac: target_mac.unwrap(),
        arp_operation: arp_operation
    };

    configure
}