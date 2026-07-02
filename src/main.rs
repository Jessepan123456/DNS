use crate::DNS::{
    Body::{
        dnsquestion::DnsQuestion,
        dnsrecord::DnsRecord,
        querytype::QueryType::{self},
    },
    dnspacket::DnsPacket,
    packetbuffer::BytePacketBuffer,
};
use std::{error::Error, io, net::UdpSocket};

mod DNS;

fn main() -> Result<(), Box<dyn Error>> {
    //qname user input
    println!("Enter a url: ");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to get input");

    let mut parts = input.split_whitespace();

    // Perform an A query for google.com
    let qname = parts.next().expect("Failed to get name");
    let qtype = match parts.next().expect("Failed to get type") {
        "A" => QueryType::A,
        "NS" => QueryType::NS,
        "CNAME" => QueryType::CNAME,
        "MX" => QueryType::MX,
        "AAAA" => QueryType::AAAA,
        _ => QueryType::UNKNOWN(0),
    };

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

    let res_packet = DnsPacket::from_buffer(&mut res_buffer)?;

    match choice().trim() {
        "1" => {
            choice_1(res_packet);
        }
        "2" => {
            choice_2(res_packet);
        }
        _ => {}
    }
    Ok(())
}

fn print(rec: DnsRecord) {
    match rec {
        DnsRecord::A { domain, addr, ttl } => {
            println!("{} -> {} (TTL: {})", domain, addr, ttl)
        }
        DnsRecord::NS { domain, host, ttl } => {
            println!("{} -> {} (TTL: {})", domain, host, ttl)
        }
        DnsRecord::CNAME { domain, host, ttl } => {
            println!("{} -> {} (TTL: {})", domain, host, ttl)
        }
        DnsRecord::MX {
            domain,
            priority,
            host,
            ttl,
        } => {
            println!("{} -> {} {} (TTL: {})", domain, host, priority, ttl)
        }
        DnsRecord::AAAA { domain, addr, ttl } => {
            println!("{} -> {} (TTL: {})", domain, addr, ttl)
        }
        DnsRecord::UNKNOWN {
            domain,
            qtype,
            data_len,
            ttl,
        } => {
            println!("{} {}, (length: {}, TTL {})", domain, qtype, data_len, ttl)
        }
    }
}

fn choice() -> String {
    println!("1 for debug, 2 for normal");
    let mut debug = String::new();
    io::stdin()
        .read_line(&mut debug)
        .expect("Failed to get user input");
    return debug;
}

fn choice_1(res_packet: DnsPacket) {
    // Parse the packet and print the response
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
}

fn choice_2(res_packet: DnsPacket) {
    println!(
        "Header:
        ID: {}
        Response: {}
        Recursion Available: {}
        Questions: {}
        Answers: {}
        Authorities: {}
        ",
        res_packet.header.id,
        res_packet.header.response,
        res_packet.header.recursion_avaiable,
        res_packet.header.questions,
        res_packet.header.answers,
        res_packet.header.authoritative_entries,
    );

    println!("Questions:");
    for q in res_packet.questions {
        println!("-{} {:?}", q.name, q.qtype);
    }

    println!("Answers:");
    for rec in res_packet.answers {
        print(rec);
    }
    if !res_packet.authorities.is_empty() {
        println!("Authorities:");
        for rec in res_packet.authorities {
            print(rec);
        }
    }
    if !res_packet.resources.is_empty() {
        println!("Additionals:");
        for rec in res_packet.resources {
            print(rec);
        }
    }
}
