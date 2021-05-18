extern crate clap;
extern crate num_cpus;

use clap::{Arg, App};
use std::net::{SocketAddr, TcpStream};
use std::time::Duration;

fn main() {
    let matches = App::new("Ports Open")
                          .version("1.0")
                          .author("Langston Graham <langstongraham@gmail.com>")
                          .about("Check the status of open ports at given ip for given range")
                          .arg(Arg::with_name("ip")
                               .help("The ip for port scan")
                               .required(true)
                               .index(1))
                          .arg(Arg::with_name("port_min")
                               .help("The start port for the scan")
                               .required(true)
                               .index(2))
                          .arg(Arg::with_name("port_max")
                               .help("The end port for the scan")
                               .required(true)
                               .index(3))
                          .get_matches();

    let ip = matches.value_of("ip").unwrap();
    let port_min = matches.value_of("port_min").unwrap();
    let port_max = matches.value_of("port_max").unwrap();

    println!("IP: {} Ports: {} - {}", ip, port_min, port_max);

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
