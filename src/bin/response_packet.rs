//Server sends back answers
use std::fs;
use std::io::Write;
use std::net::UdpSocket;

fn main() {
    let query = match fs::read("query_packet.txt") {
        Ok(q) => q,
        Err(e) => {
            println!("Failed to read query_packet, {}", e);
            return;
        }
    };

    let socket = match UdpSocket::bind("0.0.0.0:0") {
        Ok(s) => s,
        Err(e) => {
            println!("Failed to bind to port, {}", e);
            return;
        }
    };
    if let Err(e) = socket.send_to(&query, "8.8.8.8:53") {
        println!("Failed to sent to port, {}", e);
    }

    let mut buf = [0u8; 512];
    let (size, _) = match socket.recv_from(&mut buf) {
        Ok(s) => s,
        Err(e) => {
            println!("Failed to recv from, {}", e);
            return;
        }
    };

    let mut file = match fs::File::create("response_packet.txt") {
        Ok(f) => f,
        Err(e) => {
            println!("Failed to create file, {}", e);
            return;
        }
    };
    if let Err(e) = file.write_all(&buf[..size]) {
        println!("Failed to write, {}", e);
    }
}
