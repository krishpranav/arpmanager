# arpmanager
A rust tool to sending arp packets

## Usage:
```
arpmanager 0.1.0
Krisna Pranav

Send an arp packet

USAGE:
    arpmanager [FLAGS] --interface <interface> --source-ip <source_ip> --source-mac <source_mac> --target-ip <target_ip> --target-mac <target_mac>

FLAGS:
        --reply      Send an arp reply packet
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -i, --interface <interface>      Provide the interface to be used to send packets
        --source-ip <source_ip>      Set the source ip
        --source-mac <source_mac>    Set the source mac address
        --target-ip <target_ip>      Set the target ip
        --target-mac <target_mac>    Set the target mac address
```