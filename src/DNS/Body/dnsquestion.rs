//Dns questions entries
use crate::DNS::Body::querytype::QueryType;
use crate::DNS::packetbuffer::BytePacketBuffer;

#[derive(Debug, Clone, PartialEq, Eq)]
///DNS question struct
pub struct DnsQuestion {
    pub name: String,
    pub qtype: QueryType,
}

impl DnsQuestion {
    //Create a single Dns question
    pub fn new(name: String, qtype: QueryType) -> DnsQuestion {
        DnsQuestion { name, qtype }
    }

    //Reads the Dns question
    pub fn read(&mut self, buffer: &mut BytePacketBuffer) -> Result<(), String> {
        buffer.read_qname(&mut self.name)?;
        self.qtype = QueryType::from_num(buffer.read_u16()?);
        let _ = buffer.read_u16()?;

        Ok(())
    }

    //Writes the Dns question with domain name, qtype, qclass
    pub fn write(&self, buffer: &mut BytePacketBuffer) -> Result<(), String> {
        buffer.write_qname(&self.name)?;

        let typenum = self.qtype.to_num();
        buffer.write_u16(typenum)?;
        buffer.write_u16(1)?;

        Ok(())
    }
}
