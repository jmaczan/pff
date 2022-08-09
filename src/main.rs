use icmp;
use std::net::ToSocketAddrs;
use std::net::{IpAddr, Ipv4Addr};
use std::time::SystemTime;

fn main() {
    let mut addrs_iter = "cloudflare.com:443".to_socket_addrs().unwrap();
    let raw_ip = addrs_iter.next().unwrap().to_string();
    let ip = raw_ip.split(".").collect::<Vec<_>>();
    let ip_to_ping = IpAddr::V4(Ipv4Addr::new(
        String::from(ip[0]).parse::<u8>().unwrap(),
        String::from(ip[1]).parse::<u8>().unwrap(),
        String::from(ip[2]).parse::<u8>().unwrap(),
        String::from(ip[3].split(":").collect::<Vec<_>>()[0])
            .parse::<u8>()
            .unwrap(),
    ));
    let icmp_before_connect_timestamp = SystemTime::now();
    let icmp_socket = icmp::IcmpSocket::connect(ip_to_ping);
    let icmp_after_connect_timestamp = SystemTime::now();
    match &icmp_socket {
        Ok(_) => println!(
            "It took {:?} seconds to connect with server",
            icmp_after_connect_timestamp
                .duration_since(icmp_before_connect_timestamp)
                .unwrap()
                .as_secs_f64()
        ),
        Err(err) => {
            println!("Socket connection failed. {}", err);
            return;
        }
    }
    let ping_payload: &[u8] = &[1, 2]; // meaningless payload
    let icmp_before_ping_timestamp = SystemTime::now();
    let ping_result = icmp_socket.unwrap().send(ping_payload);
    let icmp_after_ping_timestamp = SystemTime::now();
    match ping_result {
        Ok(_) => {
            println!(
                "Ping took {:?} seconds",
                icmp_after_ping_timestamp
                    .duration_since(icmp_before_ping_timestamp)
                    .unwrap()
                    .as_secs_f64()
            );
        }
        Err(_) => {
            println!(
                "Ping failed. It took {:?} seconds",
                icmp_after_ping_timestamp
                    .duration_since(icmp_before_ping_timestamp)
                    .unwrap()
                    .as_secs_f64()
            );
            return;
        }
    }
}
