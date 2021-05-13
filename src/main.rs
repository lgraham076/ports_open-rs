use std::net::{SocketAddr, TcpStream};
use std::time::Duration;

extern crate num_cpus;

fn main() {
    let total_threads = num_cpus::get();
    println!("{} threads available", total_threads);

    let timeout = Duration::from_secs(1);
    let ip = [127, 0, 0, 1];
    for port in 45900..46000 {
        let addr = SocketAddr::from((ip, port));
        if TcpStream::connect_timeout(&addr, timeout).is_ok() {
            println!("Connected to server!");
        } else {
            println!("Unable to connect to server!");
        }
    }
    
}
