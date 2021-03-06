extern crate clap;
extern crate num_cpus;

use clap::{App, Arg};
use std::collections::HashSet;
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

    let ip: IpAddr = matches.value_of("ip")
                    .expect("Unable to gather ip")
                    .parse::<IpAddr>()
                    .expect("Unable to convert to IP address");
    let mut port_min: u16 = matches.value_of("port_min")
                            .expect("Unable to gather minimum port")
                            .parse()
                            .expect("Unable to interpret minimum port");
    let mut port_max: u16 = matches.value_of("port_max")
                            .expect("Unable to gather maximum port")
                            .parse()
                            .expect("Unable to interpert maximum port");

    // Verify min and max in appropriate order for range
    if port_min > port_max {
        swap(&mut port_min, &mut port_max);
    }

    // Verify port values not too high
    let max_port_value: u16 = 65535;
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

    let mut open_ports: HashSet<u16> = HashSet::new();
    let mut closed_ports: HashSet<u16> = HashSet::new();

    // Check each port in range
    for port in port_min..port_max {
        let addr = SocketAddr::from((ip, port));
        // Attempt connection...
        if TcpStream::connect_timeout(&addr, timeout).is_ok() {
            // Connection succeeded, port is open
            open_ports.insert(port);
        } else {
            // Connection failed, port is closed
            closed_ports.insert(port);
        }
    }

    println!("Open Ports: {}", open_ports.len());
    for port in open_ports {
        println!("\t{}", port);
    }
    println!("Closed Ports: {}", closed_ports.len());
    
}
