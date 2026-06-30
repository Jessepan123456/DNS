//Use for manipulating the packets in bits

/// Packet Buffer Struct
pub struct BytePacketBuffer {
    pub buf: [u8; 512],
    pub pos: usize,
}

impl BytePacketBuffer {
    //  Fresh new buffer for holding packet content
    /// Field for keeping track of where we are
    pub fn new() -> BytePacketBuffer {
        BytePacketBuffer {
            buf: [0; 512],
            pos: 0,
        }
    }

    /// Current position within buffer
    pub fn pos(&self) -> usize {
        self.pos
    }

    /// Step the buffer position forward a specific number of steps
    pub fn step(&mut self, steps: usize) -> Result<(), String> {
        self.pos += steps;

        Ok(())
    }

    /// Change the buffer position
    pub fn seek(&mut self, pos: usize) -> Result<(), String> {
        self.pos = pos;

        Ok(())
    }

    /// Read a single byte and move the position one step forward
    pub fn read(&mut self) -> Result<u8, &str> {
        if self.pos >= 512 {
            return Err("End of buffer1".into());
        }
        let res = self.buf[self.pos];
        self.pos += 1;

        Ok(res)
    }

    /// Get a single byte, without changing the position
    pub fn get(&mut self, pos: usize) -> Result<u8, &str> {
        if pos >= 512 {
            return Err("End of buffer2".into());
        }
        Ok(self.buf[pos])
    }

    /// Get a Range of bytes
    pub fn get_range(&mut self, start: usize, len: usize) -> Result<&[u8], &str> {
        if start + len >= 512 {
            return Err("End of buffer3".into());
        }
        Ok(&self.buf[start..start + len as usize])
    }

    /// Read two bytes, move forward two steps
    pub fn read_u16(&mut self) -> Result<u16, String> {
        let res = ((self.read()? as u16) << 8) | (self.read()? as u16);

        Ok(res)
    }

    /// Read four bytes, move forward four steps
    pub fn read_u32(&mut self) -> Result<u32, String> {
        let res = ((self.read()? as u32) << 24)
            | ((self.read()? as u32) << 16)
            | ((self.read()? as u32) << 8)
            | ((self.read()? as u32) << 0);

        Ok(res)
    }

    /// Read qname
    /// Reading domains, taking labels.
    /// Example: [3]www[6]google[3]com[0] and append
    /// www.google.com to outstr.
    pub fn read_qname(&mut self, outstr: &mut String) -> Result<(), String> {
        let mut pos = self.pos();

        //track whether or not we've jumped
        let mut jumped = false;
        let max_jumps = 5;
        let mut jumps_performed = 0;

        let mut delim = "";
        loop {
            //DNS Packet safety
            if jumps_performed > max_jumps {
                return Err(format!("Limit of {} jumps exceeded", max_jumps).into());
            }

            //labels start with a length byte
            let len = self.get(pos)?;

            if (len & 0xC0) == 0xC0 {
                //Update the buffer position to a point past the current
                //label.
                if !jumped {
                    self.seek(pos + 2)?;
                }

                // Read, calc offset and perform the jump by
                // updating our local position variable
                let b2 = self.get(pos + 1)? as u16;
                let offset = (((len as u16) ^ 0xC0) << 8) | b2;
                pos = offset as usize;

                // Indicate that a jump was performed
                jumped = true;
                jumps_performed += 1;

                continue;
            }
            // base scenario, reading single label and appending
            else {
                //Move a single byte forward
                pos += 1;

                //Check if the label is empty
                if len == 0 {
                    break;
                }

                //Append the delimilter to our output buffer
                outstr.push_str(delim);

                //Extract the actual ASCII and append
                let str_buffer = self.get_range(pos, len as usize)?;
                outstr.push_str(&String::from_utf8_lossy(str_buffer).to_lowercase());

                delim = ".";

                //Move forward the full length of the label

                pos += len as usize;
            }
        }

        if !jumped {
            self.seek(pos)?;
        }

        Ok(())
    }

    // Write Packets

    /// Write function
    pub fn write(&mut self, val: u8) -> Result<(), String> {
        if self.pos >= 512 {
            return Err("End of buffer".into());
        }
        self.buf[self.pos] = val;
        self.pos += 1;
        Ok(())
    }

    /// Write one bit
    pub fn write_u8(&mut self, val: u8) -> Result<(), String> {
        self.write(val)?;

        Ok(())
    }

    //Write 2 bits
    pub fn write_u16(&mut self, val: u16) -> Result<(), String> {
        self.write((val >> 8) as u8)?;
        self.write((val & 0xFF) as u8)?;

        Ok(())
    }

    //Write 4 bits
    pub fn write_u32(&mut self, val: u32) -> Result<(), String> {
        self.write(((val >> 24) & 0xFF) as u8)?;
        self.write(((val >> 16) & 0xFF) as u8)?;
        self.write(((val >> 8) & 0xFF) as u8)?;
        self.write(((val >> 0) & 0xFF) as u8)?;

        Ok(())
    }
    
    /// Write qname
    /// write domains
    /// Example: www.google.com into [3]www[6]google[3]com[0]
    pub fn write_qname(&mut self, qname: &str) -> Result<(), String> {
        for label in qname.split(".") {
            let len = label.len();
            if len > 0x3f {
                return Err("Single label exceeds 63 chars of length".into())
            }

            self.write_u8(len as u8)?;
            for b in label.as_bytes() {
                self.write_u8(*b)?;
            }
        }

        //End of domain name
        self.write_u8(0)?;

        Ok(())
    }

    pub fn set(&mut self, pos: usize, val: u8) -> Result<(), String> {
        self.buf[pos] = val;

        Ok(())
    }

    pub fn set_u16(&mut self, pos: usize, val: u16) -> Result<(), String> {
        self.set(pos, (val >> 8) as u8)?;
        self.set(pos + 1, (val & 0xFF) as u8)?;

        Ok(())
    }
}
