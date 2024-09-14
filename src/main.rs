use clap::Parser;
use colored::*;
use fastping_rs::PingResult::{Idle, Receive};
use fastping_rs::Pinger;
use spinners::{Spinner, Spinners};
use std::net::ToSocketAddrs;
use std::time::Duration;
use thiserror::Error;

const PERMISSION_ERROR_MSG: &str = "Operation not permitted";

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, default_value_t = 3)]
    trials: u8,

    #[clap(short, long, default_value = "cloudflare.com:443")]
    domain: String,

    #[clap(short, long, default_value_t = 56)]
    payload_size: usize,
}

#[derive(Error, Debug)]
enum AppError {
    #[error("IO Error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Failed to create pinger {0}")]
    PingerCreation(String),

    #[error("Failed to create pinger due to insufficient permissions. To fix it on Linux, run: sudo setcap cap_net_raw+ep <path to pff>. On macOS, run pff with sudo")]
    InsufficientPermissions,

    #[error("No addresses found for the given domain")]
    NoAddressFound,

    #[error("Pinger error {0}")]
    PingerError(#[from] std::sync::mpsc::RecvError),
}

struct PingStats {
    average_ping: Duration,
    ping_drops: u8,
}

fn resolve_domain_ip(domain: &str) -> Result<String, AppError> {
    domain
        .to_socket_addrs()?
        .next()
        .ok_or(AppError::NoAddressFound)
        .map(|addr| addr.ip().to_string())
}

fn print_result(stats: &PingStats, trials: u8) {
    let average_ping = stats.average_ping / trials.into();

    let status = match average_ping {
        d if d < Duration::from_millis(25) => "excellent".bright_green().bold(),
        d if d < Duration::from_millis(100) => "good".green().bold(),
        d if d < Duration::from_millis(500) => "average".yellow().bold(),
        d if d < Duration::from_millis(1000) => "bad".bright_red().bold(),
        _ => "really bad".red().bold(),
    };

    match stats.ping_drops {
        n if n > trials - 1 => {
            println!(
                "\rYour Internet connection seems to be either {} or {} now ({:?}/{:?} ping requests failed)",
                "really bad".red().bold(),
                "offline".red().bold(),
                stats.ping_drops,
                trials
            );
        }
        n if n > (trials / 2) - 1 => {
            println!(
                "\rYour Internet connection seems to be {} now ({:?} ping on average), but it has stability issues ({:?}/{:?} ping requests failed)",
                status,
                average_ping,
                stats.ping_drops,
                trials
            );
        }
        _ => {
            println!(
                "\rYour Internet connection is {} now ({:?} ping on average)",
                status, average_ping
            );
        }
    }
}

fn create_pinger(
    payload_size: usize,
) -> Result<(Pinger, std::sync::mpsc::Receiver<fastping_rs::PingResult>), AppError> {
    Pinger::new(None, Some(payload_size)).map_err(|e| {
        if e.contains(PERMISSION_ERROR_MSG) {
            return AppError::InsufficientPermissions;
        }
        AppError::PingerCreation(e)
    })
}

fn test_connection(
    pinger: Pinger,
    ip: String,
    args: &Args,
    results: std::sync::mpsc::Receiver<fastping_rs::PingResult>,
) -> Result<PingStats, AppError> {
    pinger.add_ipaddr(&ip);
    pinger.run_pinger();

    let mut stats = PingStats {
        average_ping: Duration::new(0, 0),
        ping_drops: 0,
    };

    let mut spinner = Spinner::new(Spinners::Dots9, "Examining your connection".into());

    for _ in 0..args.trials {
        match results.recv()? {
            Idle { addr: _ } => stats.ping_drops += 1,
            Receive { addr: _, rtt } => stats.average_ping = stats.average_ping.saturating_add(rtt),
        }
    }

    spinner.stop();

    Ok(stats)
}

fn main() -> Result<(), AppError> {
    let args = Args::parse();
    let (pinger, results) = create_pinger(args.payload_size)?;

    let ip = resolve_domain_ip(&args.domain)?;

    let stats = test_connection(pinger, ip, &args, results)?;

    print_result(&stats, args.trials);

    Ok(())
}
