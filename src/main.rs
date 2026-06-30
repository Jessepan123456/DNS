use crate::DNS::Body::dnsquestion::DnsQuestion;
use crate::DNS::Body::querytype::QueryType;
use crate::DNS::{dnspacket::DnsPacket, packetbuffer::BytePacketBuffer};
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::net::UdpSocket;

mod DNS;

fn main() -> Result<(), Box<dyn Error>> {
    // Perform an A query for google.com
    let qname = "cloudflare.com";
    let qtype = QueryType::AAAA;
    
    // Using googles public DNS server
    let server = ("8.8.8.8", 53);

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

    // Parse the packet and print the response
    let res_packet = DnsPacket::from_buffer(&mut res_buffer)?;
    println!("{:#?}", res_packet.header);

    for q in res_packet.questions {
        println!("{:#?}", q);
    }

    for rec in res_packet.answers {
        println!("{:#?}", rec);
    }

    for rec in res_packet.authorities {
        println!("{:#?}", rec);
    }

    for rec in res_packet.resources {
        println!("{:#?}", rec);
    }

    Ok(())
}
