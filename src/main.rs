use colored::*;
use fastping_rs::PingResult::{Idle, Receive};
use fastping_rs::Pinger;
use icmp::IcmpSocket;
use spinners::{Spinner, Spinners};
use std::cmp::Ordering;
use std::io::{stdout, Write};
use std::net::ToSocketAddrs;
use std::net::{IpAddr, Ipv4Addr};
use std::time::Duration;
use std::time::SystemTime;

fn resolve_domain_ip(domain: &str) -> String {
    let addrs_iter = domain.to_socket_addrs();
    match &addrs_iter {
        Ok(_) => {}
        Err(err) => {
            panic!(
                "An error occured when resolving a domain name: \"{}\"",
                err.to_string().bright_red()
            );
        }
    }
    let raw_ip = addrs_iter.unwrap().next().unwrap().to_string();
    let ip = raw_ip.split(":").collect::<Vec<_>>();
    ip[0].to_string()
}

fn print_average_ping(string: ColoredString, average_ping: Duration) {
    print!(
        "\r{} ({:?})                          ",
        string, average_ping
    );
    println!();
}

fn print_result(average_ping: Duration, total_trials: u32) {
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
    let mut spinner = Spinner::new(Spinners::Dots9, "I'm checking your connection".into());
    loop {
        if trials_left == 0 {
            break;
        }
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
    spinner.stop();

    print_result(average_ping, total_trials);
}
