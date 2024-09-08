use colored::*;
use fastping_rs::PingResult::{Idle, Receive};
use fastping_rs::Pinger;
use spinners::{Spinner, Spinners};
use std::cmp::Ordering;
use std::io::Error;
use std::net::ToSocketAddrs;
use std::time::Duration;

const TOTAL_TRIALS: u8 = 3;

fn resolve_domain_ip(domain: &str) -> Result<String, Error> {
    domain
        .to_socket_addrs()?
        .next()
        .ok_or_else(|| {
            Error::new(
                std::io::ErrorKind::NotFound,
                "Couldn't resolve domain IP to check your internet connection",
            )
        })
        .map(|addr| addr.ip().to_string())
}

fn print_average_ping(string: ColoredString, average_ping: Duration, ping_drops: u8) {
    if ping_drops > TOTAL_TRIALS - 1 {
        println!(
            "\rYour Internet connection seems to be either {} or {} now ({:?}/{:?} ping requests failed)",
            "really bad".red().bold(),
            "offline".red().bold(),
            ping_drops,
            TOTAL_TRIALS
        );
    } else if ping_drops > ((TOTAL_TRIALS / 2) - 1) {
        println!(
            "\rYour Internet connection seems to be {} now ({:?} ping on average), but it has stability issues ({:?}/{:?} ping requests failed)",
            string,
            average_ping,
            ping_drops,
            TOTAL_TRIALS
        );
    } else {
        println!(
            "\rYour Internet connection is {} now ({:?} ping on average)",
            string, average_ping
        );
    }
}

fn print_result(average_ping: Duration, ping_drops: u8) {
    let average_ping = average_ping / TOTAL_TRIALS.into();

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
            if err == "Operation not permitted (os error 1)" {
                println!("I couldn't perform your internet examination, due to lack of CAP_NET_RAW capabilities. To fix it, run:\nsudo setcap cap_net_raw+ep <path to pff>");
                return;
            }
            println!(
                "Are you connected to the Internet? I couldn't perform your internet examination. Technical reason: \"{}\"",
                err
            );
            return;
        }
    };
    let ip = match resolve_domain_ip("cloudflare.com:443") {
        Ok(ip) => ip,
        Err(err) => {
            println!(
                "Are you connected to the Internet?\nI couldn't perform your internet examination due to failed domain resolution. Technical reason: \"{}\"",
                err.to_string()
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
    let mut spinner = Spinner::new(Spinners::Dots9, "I'm examining your connection".into());
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
