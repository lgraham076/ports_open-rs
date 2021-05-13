use std::net::{SocketAddr, TcpStream};
use std::time::Duration;

extern crate num_cpus;

fn main() {
    // Check number of threads available
    let total_threads = num_cpus::get();
    println!("{} threads available", total_threads);

    // Set port check timeout
    let timeout = Duration::from_secs(1);
    // Set ip to localhost
    let ip = [127, 0, 0, 1];
    let port_range = 45900..46000;
    // Check each port in range
    for port in port_range {
        let addr = SocketAddr::from((ip, port));
        // Attempt connection...
        if TcpStream::connect_timeout(&addr, timeout).is_ok() {
            // Connection succeeded, port is open
            println!("Connected to server!");
        } else {
            // Connection failed, port is closed
            println!("Unable to connect to server!");
        }
    }
    
}
