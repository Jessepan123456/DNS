use crate::DNS::{
    Body::{
        dnsquestion::DnsQuestion,
        querytype::QueryType::{self},
    },
    dnspacket::DnsPacket,
    helper::user_input,
    packetbuffer::BytePacketBuffer,
};
use std::{error::Error, net::UdpSocket};

#[allow(non_snake_case)]
mod DNS;

fn main() -> Result<(), Box<dyn Error>> {
    //qname user input
    println!("Enter a url, type, DNS server, and port(Put space between them): ");

    let binding = user_input();
    //Default Settings
    let mut qname = "google.com";
    let mut qtype = QueryType::A;
    let mut server_info = "8.8.8.8";
    let mut port = 53;
    if !binding.is_empty() {
        let mut parts = binding.split_whitespace();

        // Perform an A query for google.com
        qname = parts.next().expect("Failed to get name");

        qtype = match parts.next().expect("Failed to get type") {
            "A" => QueryType::A,
            "NS" => QueryType::NS,
            "CNAME" => QueryType::CNAME,
            "MX" => QueryType::MX,
            "TXT" => QueryType::TXT,
            "AAAA" => QueryType::AAAA,
            _ => QueryType::UNKNOWN(0),
        };
        server_info = match parts.next() {
            Some(i) => i,
            None => {
                println!("Please provide a server");
                return Ok(());
            }
        };
        port = match parts.next() {
            Some(p) => p.parse().expect("Failed to parse"),
            None => {
                println!("Please provide a port");
                return Ok(());
            }
        }
    }
    // Using googles public DNS server
    let server = (server_info, port);

    // Bind a UDP socket to an arbitrary port
    let socket = UdpSocket::bind(("0.0.0.0", 43210))?;

    //Build our query packet. 'recursion_desired' flag set important
    let mut packet = DnsPacket::new();

    packet.header.id = 6666;
    packet.header.questions = 1;
    packet.header.recursion_desired = true;
    packet
        .questions
        .push(DnsQuestion::new(qname.to_string(), qtype));

    //Use our new write method to write the packet to a buffer
    let mut req_buffer = BytePacketBuffer::new();
    packet.write(&mut req_buffer)?;

    // send it off to the server using our socket
    socket.send_to(&req_buffer.buf[0..req_buffer.pos], server)?;

    // Prepare for receiving the response and ask the socket to write the response into our buffer
    let mut res_buffer = BytePacketBuffer::new();
    socket.recv_from(&mut res_buffer.buf)?;

    let res_packet = DnsPacket::from_buffer(&mut res_buffer)?;

    match DNS::helper::choice().trim() {
        "1" => {
            DNS::helper::choice_1(res_packet);
        }
        "2" => {
            DNS::helper::choice_2(res_packet);
        }
        _ => {}
    }
    Ok(())
}
