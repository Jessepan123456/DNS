use crate::DNS::{Body::dnsrecord::DnsRecord, dnspacket::DnsPacket};
use std::io;

pub fn print(rec: DnsRecord) {
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
        DnsRecord::TXT { domain,text, ttl } => {
            println!("{}, {}, (TTL: {})", domain, text, ttl)
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

pub fn choice() -> String {
    println!("1 for debug, 2 for normal");
    let mut debug = String::new();
    io::stdin()
        .read_line(&mut debug)
        .expect("Failed to get user input");
    return debug;
}

pub fn choice_1(res_packet: DnsPacket) {
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

pub fn choice_2(res_packet: DnsPacket) {
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

pub fn user_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to get input");
    return input.trim().to_string();
}
