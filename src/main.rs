use colored::*;
use fastping_rs::PingResult::{Idle, Receive};
use fastping_rs::Pinger;
use spinners::{Spinner, Spinners};
use std::cmp::Ordering;
use std::io::Error;
use std::net::ToSocketAddrs;
use std::time::Duration;

const TOTAL_TRIALS: u32 = 3;

fn resolve_domain_ip(domain: &str) -> Result<String, Error> {
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

fn print_average_ping(string: ColoredString, average_ping: Duration, ping_drops: u32) {
    if ping_drops > TOTAL_TRIALS - 1 {
        print!(
            "\rYour Internet connection seems to be either {} or {} ({:?}/{:?} ping requests failed)",
            "really bad".red().bold(),
            "offline".red().bold(),
            ping_drops,
            TOTAL_TRIALS
        );
    } else if ping_drops > ((TOTAL_TRIALS / 2) - 1) {
        print!(
            "\rYour Internet connection seems to be {}, but having stability issues ({:?}/{:?} ping requests failed)",
            string,
            ping_drops,
            TOTAL_TRIALS
        );
    } else {
        print!(
            "\rYour Internet connection is {} ({:?} ping on average)",
            string, average_ping
        );
    }
    println!();
}

fn print_result(average_ping: Duration, ping_drops: u32) {
    let average_ping = average_ping / TOTAL_TRIALS;

    if (average_ping).cmp(&Duration::from_millis(25)) == Ordering::Less {
        print_average_ping("excellent".bright_green().bold(), average_ping, ping_drops);
    } else if average_ping.cmp(&Duration::from_millis(100)) == Ordering::Less {
        print_average_ping("good".green().bold(), average_ping, ping_drops);
    } else if average_ping.cmp(&Duration::from_millis(500)) == Ordering::Less {
        print_average_ping("average".yellow().bold(), average_ping, ping_drops);
    } else if average_ping.cmp(&Duration::from_millis(1000)) == Ordering::Less {
        print_average_ping("bad".bright_red().bold(), average_ping, ping_drops);
    } else {
        print_average_ping("really bad".red().bold(), average_ping, ping_drops);
    }
}

fn create_pinger() -> Result<
    (
        fastping_rs::Pinger,
        std::sync::mpsc::Receiver<fastping_rs::PingResult>,
    ),
    String,
> {
    Pinger::new(None, Some(56))
}

fn main() {
    let (pinger, results) = match create_pinger() {
        Ok((pinger, results)) => (pinger, results),
        Err(err) => {
            println!(
                "Are you connected to the Internet? I couldn't perform your internet examination. Technical reason: \"{}\"",
                err.bright_red()
            );
            return;
        }
    };
    let ip = match resolve_domain_ip("cloudflare.com:443") {
        Ok(ip) => ip,
        Err(err) => {
            println!(
                "Are you connected to the Internet? I couldn't perform your internet examination due to failed domain resolution. Technical reason: \"{}\"",
                err.to_string().bright_red()
            );
            return;
        }
    };

    pinger.add_ipaddr(&ip);
    pinger.run_pinger();

    let mut average_ping = Duration::new(0, 0);
    let mut trials_left = TOTAL_TRIALS;
    let mut ping_drops = 0;
    let mut ping_fails = 0;
    let mut spinner = Spinner::new(Spinners::Dots9, "I'm examining your ping".into());
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
            Err(_) => {
                ping_fails += 1;
            }
        }
    }
    spinner.stop();

    print_result(average_ping, ping_drops + ping_fails);
}
