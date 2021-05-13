use std::net::{SocketAddr, TcpStream};
use std::time::Duration;

fn main() {
    let timeout = Duration::from_secs(1);
    let ip = [127, 0, 0, 1];
    for port in 45900..46000 {
        let addr = SocketAddr::from((ip, port));
        if let Ok(stream) = TcpStream::connect_timeout(&addr, timeout) {
            println!("Connected to server!");
        } else {
            println!("Unable to connect to server!");
        }
    }
    
}
