# Rust DNS Reader/Writer

A learning project for understanding the DNS Packets by reading and writing

## Features
- Read (parse) DNS packets from raw bytes
- Write DNS pakcets into raw bytes
- Supports DNS headers, questions, and basic resource records
- Command line interface
- Supports DNS Record types:
  - A
  - NS
  - CNAME
  - MX
  - AAAA
  - TXT
  
## Credit
This project is based on "Building a DNS server in Rust" by EmillHernvall:
https://github.com/EmilHernvall/dnsguide.git 

## Addons
- Support more DNS record types
- Even better Packet Validation
- Unit Test
- Custom lookup
- Adding a custom working Server
