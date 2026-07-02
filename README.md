# Rust DNS Reader/Writer

A learning project for understanding the DNS Packets by reading and writing

## Features
- Read (parse) DNS packets from raw bytes
- Write DNS pakcets into raw bytes
- Supports DNS headers, questions, and basic resource records
- Supports DNS Record types:
  - A
  - NS
  - CNAME
  - MX
  - AAAA
  
## Credit
This project is based on "Building a DNS server in Rust" by EmillHernvall:
https://github.com/EmilHernvall/dnsguide.git 

## Addons
- Support more DNS record types
- Better packet printing
- Error Checking
- Command line interface
- Configurable DNS server
- Packet validation
