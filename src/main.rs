use colored::*;
use fastping_rs::PingResult::{Idle, Receive};
use fastping_rs::Pinger;
use icmp::IcmpSocket;
use std::cmp::Ordering;
use std::net::ToSocketAddrs;
use std::net::{IpAddr, Ipv4Addr};
use std::time::Duration;
use std::time::SystemTime;
use std::{
    io::{stdout, Write},
    thread::sleep,
};
use spinners::{Spinner, Spinners};

fn resolve_domain_ip(domain: &str) -> String {
    let addrs_iter = domain.to_socket_addrs();
    match &addrs_iter {
        Ok(_) => {}
        Err(err) => {
            panic!("An error occured when resolving a domain name: \"{}\"", err);
        }
    }
    let raw_ip = addrs_iter.unwrap().next().unwrap().to_string();
    let ip = raw_ip.split(":").collect::<Vec<_>>();
    // let ip = raw_ip.split(".").collect::<Vec<_>>();
    // let ip_to_ping = IpAddr::V4(Ipv4Addr::new(
    //     String::from(ip[0]).parse::<u8>().unwrap(),
    //     String::from(ip[1]).parse::<u8>().unwrap(),
    //     String::from(ip[2]).parse::<u8>().unwrap(),
    //     String::from(ip[3].split(":").collect::<Vec<_>>()[0])
    //         .parse::<u8>()
    //         .unwrap(),
    // ));
    // ip_to_ping
    ip[0].to_string()
}

fn connect_icmp_socket(ip_to_ping: IpAddr) -> IcmpSocket {
    let icmp_before_connect_timestamp = SystemTime::now();
    let icmp_socket = IcmpSocket::connect(ip_to_ping);
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
            panic!("Socket connection failed. {}", err);
        }
    }

    icmp_socket.unwrap()
}

fn ping(mut icmp_socket: IcmpSocket, ping_payload: &[u8]) {
    let icmp_before_ping_timestamp = SystemTime::now();
    let ping_result = icmp_socket.send(ping_payload);
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

fn print_average_ping(string: ColoredString, average_ping: Duration) {
    print!("\r{} ({:?})", string, average_ping);
    println!();
}

fn main() {
    let (pinger, results) = match Pinger::new(None, Some(56)) {
        Ok((pinger, results)) => (pinger, results),
        Err(e) => panic!("Error creating pinger: {}", e),
    };
    let ip = resolve_domain_ip("cloudflare.com:443");
    pinger.add_ipaddr(&ip);
    pinger.run_pinger();

    let mut average_ping = Duration::new(0, 0);
    let total_trials = 3;
    let mut trials_left = total_trials;
    let mut stdout = stdout();
    loop {
        if trials_left == 0 {
            break;
        }
        print!("\r{}... {}", "Measuring".blue().italic(), trials_left.to_string().magenta());
        stdout.flush().unwrap();
        trials_left -= 1;
        match results.recv() {
            Ok(result) => match result {
                Idle { addr } => {
                    println!("Idle Address {}.", addr);
                }
                Receive { addr: _, rtt } => {
                    average_ping = average_ping.saturating_add(rtt);
                }
            },
            Err(_) => panic!("Worker threads disconnected before the solution was found!"),
        }
    }

    let average_ping = average_ping / total_trials;

    if (average_ping).cmp(&Duration::from_millis(25)) == Ordering::Less {
        print_average_ping("Excellent".bright_green().bold(), average_ping);
    } else if average_ping.cmp(&Duration::from_millis(100)) == Ordering::Less {
        print_average_ping("Good".green().bold(), average_ping);
    } else if average_ping.cmp(&Duration::from_millis(500)) == Ordering::Less {
        print_average_ping("Average".yellow().bold(), average_ping);
    } else if average_ping.cmp(&Duration::from_millis(1000)) == Ordering::Less {
        print_average_ping("Bad".bright_red().bold(), average_ping);
    } else {
        print_average_ping("Really bad".red().bold(), average_ping);
    }

    // let ip_to_ping = resolve_domain_ip("cloudflare.com:443");
    // let icmp_socket = connect_icmp_socket(ip_to_ping);
    // let ping_payload: &[u8] = &[1, 2]; // a meaningless payload
    // ping(icmp_socket, ping_payload);
}
