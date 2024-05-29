use std::collections::HashMap;
use std::io::Result;
use std::net::{IpAddr, UdpSocket};
use std::str::FromStr;
use std::time::{Duration, Instant};
use wol::{send_wol, MacAddr};

/// mac address size of bytes
const MAC_ADDR_SIZE: usize = 6;

/// wol port could be 0/7/9, use 9 here
const WOL_PORT: u16 = 9;

const DUPLICATE_TIMEFRAME: Duration = Duration::from_secs(10);

/// extract the mac address from a magic packet.
fn extract_mac_addr(packet: &[u8]) -> Option<MacAddr> {
    if packet.len() < 6 + 16 * MAC_ADDR_SIZE {
        return None;
    }

    for i in 0..6 {
        if packet[i] != 0xff {
            return None;
        }
    }

    let mut mac_addr = MacAddr([0; MAC_ADDR_SIZE]);
    for i in 0..16 {
        for j in 0..MAC_ADDR_SIZE {
            mac_addr.0[j] = packet[6 + i * MAC_ADDR_SIZE + j];
        }
    }

    Some(mac_addr)
}

fn main() -> Result<()> {
    // bind on port 9
    let socket = UdpSocket::bind(("0.0.0.0", WOL_PORT))?;
    socket.set_broadcast(true)?;

    //  track recently sent mac addresses and timestamps
    let mut sent_packets: HashMap<String, Instant> = HashMap::new();

    println!("baridge WOL relay server listening on port {}", WOL_PORT);

    loop {
        let mut buffer = [0; 1024];
        let (size, _src_addr) = socket.recv_from(&mut buffer)?;
        let packet = &buffer[..size];

        // extract mac address from received packet
        let mac_addr = extract_mac_addr(packet);

        if let Some(mac_addr) = mac_addr {
            println!("received magic packet for MAC address: {}", mac_addr);

            let mac_addr_str = format!("{}", mac_addr);

            // check for duplicates based on timeframe
            let now = Instant::now();
            if !sent_packets.contains_key(&mac_addr_str)
                || now.duration_since(sent_packets[&mac_addr_str]) > DUPLICATE_TIMEFRAME
            {
                let bcast_addr = IpAddr::from_str("255.255.255.255").unwrap();
                let bcast_addr: Option<IpAddr> = Some(bcast_addr);

                send_wol(mac_addr, bcast_addr, None)?;
                sent_packets.insert(mac_addr_str, now);
            } else {
                // to aovid feedback loops
                println!("ignoring duplicate packet for MAC address: {}", mac_addr);
            }
        } else {
            println!("received invalid magic packet.");
        }
    }
}
