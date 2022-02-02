use clap::Parser;
use futures::{stream, StreamExt};
use std::io::{Error, ErrorKind};
use std::{
    net::{IpAddr, SocketAddr, ToSocketAddrs},
    time::Duration,
};
use tokio::net::TcpStream;

mod ports;

#[derive(Parser, Debug)]
#[clap(about, version, author)]
pub struct Args {
    #[clap(long, help = "The target to scan", required = true)]
    target: String,

    #[clap(
        short = 'c',
        long,
        default_value = "1000",
        help = "The number of concurrent scans to process"
    )]
    concurrency: String,

    #[clap(short = 'v', long, help = "Display detailed results")]
    verbose: bool,

    #[clap(short = 'p', long, default_value = "00", help = "Range of ports to scan, e.g. 0-65535")]
    // TODO revamp use here to indicate ports instad of hardcoding to all possible
    port_range: String,

    #[clap(short = 't', long, default_value = "3", help = "Connection timeout in seconds")]
    timeout: String,
}

async fn scan(target: IpAddr, port_range: String, concurrency: usize, timeout: u64) {
    let ports = stream::iter(get_ports(port_range));

    ports
        .for_each_concurrent(concurrency, |port| scan_port(target, port, timeout))
        .await;
}

async fn scan_port(target: IpAddr, port: u16, timeout: u64) {
    let timeout = Duration::from_secs(timeout);
    let socket_address = SocketAddr::new(target.clone(), port);

    //TODO refactor to store port into "open ports" hashmap for cleaner display at the end
    match tokio::time::timeout(timeout, TcpStream::connect(&socket_address)).await {
        Ok(Ok(_)) => println!("{}", port),
        _ => {}
    }
}

fn get_ports(port_range: String) -> Box<dyn Iterator<Item = u16>> {
    if port_range != "00" {
        if port_range.contains("-"){
            let split: Vec<&str> = port_range.split("-").collect();
            Box::new((split[0].parse::<u16>().unwrap()..=split[1].parse::<u16>().unwrap()).into_iter())
        } else {
            // let mut port_vec: Vec<u16> = Vec::new();
            let port =  port_range.parse::<u16>().unwrap();
            // port_vec.append(port);
            // TODO try to get an iterator of a single value, so we don't hit the port twice for no reason
            Box::new((port..=port).into_iter())
        }
    } else {
        Box::new(ports::MOST_COMMON_PORTS_1002.to_owned().into_iter())
    }
}


#[tokio::main]
async fn main() -> Result<(), Error> {
    // example CLI invocations
    // cargo run -- --target=127.0.0.1
    // cargo run -- --target=nmap.scanme.org
    // cargo run -- --target=nmap.scanme.org --port_range=0-65535
    let args = Args::parse();

    let concurrency = args.concurrency.parse::<usize>().unwrap_or(1002);
    let port_range = args.port_range;
    let target = args.target;
    let timeout = args.timeout.parse::<u64>().unwrap_or(3);

    println!("Provided target: {}", target);
    let socket_addresses: Vec<SocketAddr> = format!("{}:0", target).to_socket_addrs()?.collect();
    println!("Scanning target IP address: {}", socket_addresses[0].ip().to_string());

    if socket_addresses.is_empty() {
        return Err(Error::new(
            ErrorKind::NotFound,
            "Socket addresses list is empty",
        ));
    }

    scan(socket_addresses[0].ip(), port_range, concurrency, timeout).await;

    Ok(())
}