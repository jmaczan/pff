use icmp;
use std::net::ToSocketAddrs;
use std::net::{IpAddr, Ipv4Addr};
use std::time::SystemTime;

fn main() {
    let mut addrs_iter = "cloudflare.com:443".to_socket_addrs().unwrap();
    let raw_ip = addrs_iter.next().unwrap().to_string();
    let mut ip = raw_ip.split(".").collect::<Vec<_>>();
    ip[3] = ip[3].split(":").collect::<Vec<_>>()[0];
    println!("{:?}", ip);
    println!(
        "{} {} {} {}",
        String::from(ip[0]).parse::<u8>().unwrap(),
        String::from(ip[1]).parse::<u8>().unwrap(),
        String::from(ip[2]).parse::<u8>().unwrap(),
        String::from(ip[3].split(":").collect::<Vec<_>>()[0])
            .parse::<u8>()
            .unwrap()
    );
    let ip_to_ping = IpAddr::V4(Ipv4Addr::new(
        String::from(ip[0]).parse::<u8>().unwrap(),
        String::from(ip[1]).parse::<u8>().unwrap(),
        String::from(ip[2]).parse::<u8>().unwrap(),
        String::from(ip[3].split(":").collect::<Vec<_>>()[0])
            .parse::<u8>()
            .unwrap(),
    ));
    let icmp_before_connect_timestamp = SystemTime::now();
    let icmp_after_connect_timestamp = SystemTime::now();
    println!(
        "{:?}, {:?}, {:?}",
        icmp_before_connect_timestamp,
        icmp_after_connect_timestamp,
        icmp_after_connect_timestamp.duration_since(icmp_before_connect_timestamp)
    );
    let mut ping = icmp::IcmpSocket::connect(ip_to_ping).unwrap();
    let ping_payload: &[u8] = &[1, 2]; // random payload
    let icmp_before_ping_timestamp = SystemTime::now();
    let ping_result = ping.send(ping_payload);
    let icmp_after_ping_timestamp = SystemTime::now();
    println!(
        "{:?}, {:?}, {:?}, {:?}",
        ping_result.unwrap(),
        icmp_before_ping_timestamp,
        icmp_after_ping_timestamp,
        icmp_after_ping_timestamp.duration_since(icmp_before_ping_timestamp)
    );
}
