use std::net::{Ipv4Addr, AddrParseError};
use pnet::util::{MacAddr, ParseMacAddrErr};
use pnet::packet::arp::{ArpOperation, ArpOperations};
use clap::{Arg, App};
use config::Config;

const ABOUT: &'static str = "
Send arp packet";

