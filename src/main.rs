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

    #[clap(short = 'p', long, help = "Range of ports to scan, e.g. 0-65535")]
    // TODO revamp use here to indicate ports instad of hardcoding to all possible
    port_range: bool,

    #[clap(short = 't', long, help = "Connection timeout in seconds")]
    timeout: String,
}
async fn scan(target: IpAddr, full: bool, concurrency: usize, timeout: u64) {
    let ports = stream::iter(get_ports(full));

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

fn get_ports(full: bool) -> Box<dyn Iterator<Item = u16>> {
    // TODO revamp use here to indicate port range instad of hardcoding to all possible
    if full {
        Box::new((1..=u16::MAX).into_iter())
    } else {
        Box::new(ports::MOST_COMMON_PORTS_1002.to_owned().into_iter())
    }
}


#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = Args::parse();

    let concurrency = args.concurrency.parse::<usize>().unwrap_or(1002);
    let full = args.port_range;
    let target = args.target;
    let timeout = args.timeout.parse::<u64>().unwrap_or(3);

    let socket_addresses: Vec<SocketAddr> = format!("{}:0", target).to_socket_addrs()?.collect();

    if socket_addresses.is_empty() {
        return Err(Error::new(
            ErrorKind::NotFound,
            "Socket addresses list is empty",
        ));
    }

    scan(socket_addresses[0].ip(), full, concurrency, timeout).await;

    Ok(())
}