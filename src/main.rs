use colored::*;
use fastping_rs::PingResult::{Idle, Receive};
use fastping_rs::Pinger;
use spinners::{Spinner, Spinners};
use std::cmp::Ordering;
use std::net::ToSocketAddrs;
use std::time::Duration;

fn resolve_domain_ip(domain: &str) -> Result<String, std::io::Error> {
    let addrs_iter = domain.to_socket_addrs();
    match addrs_iter {
        Ok(_) => {
            let raw_ip = addrs_iter.unwrap().next().unwrap().to_string();
            let ip = raw_ip.split(":").collect::<Vec<_>>();
            Ok(ip[0].to_string())
        }
        Err(err) => Err(err),
    }
}

fn print_average_ping(string: ColoredString, average_ping: Duration) {
    print!(
        "\rYour Internet connection is {} ({:?} ping on average)",
        string, average_ping
    );
    println!();
}

fn print_result(average_ping: Duration, total_trials: u32, ping_drops: u32) {
    let average_ping = average_ping / total_trials;

    if (average_ping).cmp(&Duration::from_millis(25)) == Ordering::Less {
        print_average_ping("excellent".bright_green().bold(), average_ping);
    } else if average_ping.cmp(&Duration::from_millis(100)) == Ordering::Less {
        print_average_ping("good".green().bold(), average_ping);
    } else if average_ping.cmp(&Duration::from_millis(500)) == Ordering::Less {
        print_average_ping("average".yellow().bold(), average_ping);
    } else if average_ping.cmp(&Duration::from_millis(1000)) == Ordering::Less {
        print_average_ping("bad".bright_red().bold(), average_ping);
    } else {
        print_average_ping("really bad".red().bold(), average_ping);
    }
}

fn create_pinger() -> (
    fastping_rs::Pinger,
    std::sync::mpsc::Receiver<fastping_rs::PingResult>,
) {
    match Pinger::new(None, Some(56)) {
        Ok((pinger, results)) => (pinger, results),
        Err(e) => panic!("Error creating pinger: {}", e),
    }
}

fn main() {
    let (pinger, results) = create_pinger();
    let ip = match resolve_domain_ip("cloudflare.com:443") {
        Ok(ip) => ip,
        Err(err) => {
            println!(
                "An error occured when resolving a domain name: \"{}\"",
                err.to_string().bright_red()
            );
            return;
        }
    };

    pinger.add_ipaddr(&ip);
    pinger.run_pinger();

    let mut average_ping = Duration::new(0, 0);
    let total_trials = 3;
    let mut trials_left = total_trials;
    let mut spinner = Spinner::new(Spinners::Dots9, "I'm examining your ping".into());
    let mut ping_drops = 0;
    loop {
        if trials_left == 0 {
            break;
        }
        trials_left -= 1;
        match results.recv() {
            Ok(result) => match result {
                Idle { addr: _ } => {
                    ping_drops += 1;
                }
                Receive { addr: _, rtt } => {
                    average_ping = average_ping.saturating_add(rtt);
                }
            },
            Err(_) => panic!("Worker threads disconnected before the solution was found!"),
        }
    }
    spinner.stop();

    print_result(average_ping, total_trials, ping_drops);
}
