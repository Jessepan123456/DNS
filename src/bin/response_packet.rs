//Server sends back answers
use std::fs;
use std::io::Write;
use std::net::UdpSocket;

fn main() {
    let query = fs::read("query_packet.txt").unwrap();

    let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
    socket.send_to(&query, "8.8.8.8:53").unwrap();

    let mut buf = [0u8; 512];
    let (size, _) = socket.recv_from(&mut buf).unwrap();

    let mut file = fs::File::create("response_packet.txt").unwrap();
    file.write_all(&buf[..size]).unwrap();
}
