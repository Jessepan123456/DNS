//What your computer sends to the DNS server
use std::fs::File;
use std::io::Write;

fn main() {
    //Simple DNS query
    let packet: [u8; 28] = [
        0x86, 0x2a, // ID

        0x01, 0x00, // flag

        0x00, 0x01, // QDCOUNT
        0x00, 0x00, //ANCOUNT
        0x00, 0x00, //NSCOUNT
        0x00, 0x00, //ARCOUNT

        //qname: google.com
        0x06, b'g', b'o', b'o', b'g', b'l', b'e',
        0x03, b'c', b'o', b'm',
        0x00,

        0x00, 0x01, // type A
        0x00, 0x01, // class IN
    ];

    let mut file = File::create("query_packet.txt").unwrap();
    file.write_all(&packet).unwrap();
}