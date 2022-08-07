use http::Request;
use icmp;
use reqwest::blocking;
use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr};
use std::net::{SocketAddr, ToSocketAddrs};
use std::time::SystemTime;

fn main() {
    // println!("Hello, world!");
    // let request = Request::get("https://cloudflare.com").body(()).unwrap();
    // println!("{:?}", request);
    // println!("{:?}", request.headers());
    // send(request)

    let mut addrs_iter = "cloudflare.com:443".to_socket_addrs().unwrap();
    let ip = addrs_iter.next().unwrap();
    println!("{:?}", ip);

    // let client = blocking::Client::new();
    // let mut get = client.get("https://cloudflare.com");
    // let mut before = SystemTime::now();
    // let mut response = get.send();
    // let mut after = SystemTime::now();
    // println!("{:?}", after.duration_since(before));
    // get = client.get("https://cloudflare.com");
    // before = SystemTime::now();
    // response = get.send();
    // println!("{:?}", response);
    // after = SystemTime::now();
    // println!("{:?}", after.duration_since(before));
    // let get_localhost = client.get("http://localhost");
    // before = SystemTime::now();
    // response = get_localhost.send();
    // after = SystemTime::now();
    // println!("localhost {:?}", after.duration_since(before));
    //let response = reqwest::blocking::get("https://cloudflare.com")?.text(); //.json::<HashMap<String, String>>()?;

    let localhost_v4 = IpAddr::V4(Ipv4Addr::new(104, 16, 133, 229)); // this should be ip variable, but formatted
    // let localhost_v4 = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    let mut now = SystemTime::now();
    let ping = icmp::IcmpSocket::connect(localhost_v4);
    let mut after = SystemTime::now();
    println!("{:?}, {:?}, {:?}", now, after, after.duration_since(now));
    let mut ping = ping.unwrap();
    let payload: &[u8] = &[1, 2];
    let mut now = SystemTime::now();
    let result = ping.send(payload);
    let mut after = SystemTime::now();
    println!( //result is strange, because time is very short, like 39.238µs
        "{:?}, {:?}, {:?}, {:?}",
        result,
        now,
        after,
        after.duration_since(now)
    );
    let result = ping.send(&[1, 2, 3]);
    println!("{:?}", result);
}
