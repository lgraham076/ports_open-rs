extern crate clap;
extern crate num_cpus;

use clap::{App, Arg};
use std::mem::swap;
use std::net::{IpAddr, SocketAddr, TcpStream};
use std::process::exit;
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

    let ip = matches.value_of("ip").expect("Unable to gather ip")
                    .parse::<IpAddr>().expect("Unable to convert to IP address");
    let mut port_min: i32 = matches.value_of("port_min").expect("Unable to gather minimum port")
                    .parse().expect("Unable to interpret minimum port");
    let mut port_max: i32 = matches.value_of("port_max").expect("Unable to gather maximum port")
                    .parse().expect("Unable to interpert maximum port");

    // Verify min and max in appropriate order for range
    if port_min > port_max {
        swap(&mut port_min, &mut port_max);
    }
    // Verify min and max non-negative 
    if port_min < 0 || port_max < 0 {
        eprintln!("Port min and max must be non-negative");
        exit(1);
    }
    // Verify port values not too high
    let max_port_value = 65535;
    if port_min > max_port_value || port_max > max_port_value {
        eprintln!("Port min and max must be less than or equal to {}", max_port_value);
        exit(1);
    }

    println!("IP: {} Ports: {} - {}", ip, port_min, port_max);

    // Check number of threads available
    let total_threads = num_cpus::get();
    println!("{} threads available", total_threads);

    // Set port check timeout
    let timeout = Duration::from_secs(1);
    // Set ip to localhost
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
