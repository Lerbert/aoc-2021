use anyhow::{anyhow, bail, Result};
use bit_vec::BitVec;
use hex;

trait BITSPacket {
    fn version_sum(&self) -> u32;
    fn evaluate(&self) -> u128;
}

#[derive(Debug)]
struct LiteralBITSPacket {
    version: u8,
    type_id: u8,
    value: u128,
}

impl BITSPacket for LiteralBITSPacket {
    fn version_sum(&self) -> u32 {
        self.version as u32
    }

    fn evaluate(&self) -> u128 {
        self.value
    }
}

struct OperatorBITSPacket {
    version: u8,
    type_id: u8,
    _length_type_id: u8,
    _length: u32,
    subpackets: Vec<Box<dyn BITSPacket>>,
}

impl BITSPacket for OperatorBITSPacket {
    fn version_sum(&self) -> u32 {
        self.subpackets.iter().map(|p| p.version_sum()).sum::<u32>() + self.version as u32
    }

    fn evaluate(&self) -> u128 {
        let subpacket_values = self.subpackets.iter().map(|p| p.evaluate());
        match self.type_id {
            0 => subpacket_values.sum(),
            1 => subpacket_values.product(),
            2 => subpacket_values.min().expect("no subpackets"),
            3 => subpacket_values.max().expect("no subpackets"),
            5 | 6 | 7 => {
                let subpacket_values: Vec<_> = subpacket_values.collect();
                if subpacket_values.len() != 2 {
                    panic!("malformed comparison packet")
                }
                let fst = subpacket_values[0];
                let snd = subpacket_values[1];
                match self.type_id {
                    5 => (fst > snd) as u128,
                    6 => (fst < snd) as u128,
                    7 => (fst == snd) as u128,
                    _ => panic!("shouldn't reach this"),
                }
            }
            4 => panic!("this should have been a literal packet"),
            _ => panic!("unknown type_id"),
        }
    }
}

#[derive(Debug)]
struct BITSParser {
    bits: BitVec,
    offset: usize,
    length: usize,
}

impl BITSParser {
    pub fn from(bits: BitVec) -> Self {
        BITSParser {
            length: bits.len(),
            offset: 0,
            bits,
        }
    }

    pub fn parse_packet(mut self) -> Result<Box<dyn BITSPacket>> {
        self.parse_packet_helper()
    }

    fn parse_packet_helper(&mut self) -> Result<Box<dyn BITSPacket>> {
        let version = self
            .parse_number(3)
            .map_err(|e| anyhow!("Error parsing version: {}", e))? as u8;
        let type_id = self
            .parse_number(3)
            .map_err(|e| anyhow!("Error parsing type_id: {}", e))? as u8;
        if type_id == 4 {
            // Literal packet
            let value = self.parse_value()?;
            Ok(Box::new(LiteralBITSPacket {
                version,
                type_id,
                value,
            }))
        } else {
            // Operator packet
            let length_type_id = self
                .next()
                .map_err(|e| anyhow!("Error parsing length_type_id: {}", e))?;
            if length_type_id {
                // Next 11 are number of subpackets
                let subpacket_cnt = self
                    .parse_number(11)
                    .map_err(|e| anyhow!("Error parsing subpacket_cnt: {}", e))?;
                let subpackets = (0..subpacket_cnt)
                    .map(|_| self.parse_packet_helper())
                    .collect::<Result<Vec<_>>>()
                    .map_err(|e| anyhow!("Error parsing subpacket: {}", e))?;
                Ok(Box::new(OperatorBITSPacket {
                    version,
                    type_id,
                    _length_type_id: length_type_id as u8,
                    _length: subpacket_cnt,
                    subpackets,
                }))
            } else {
                // Next 15 are total length of subpackets
                let subpacket_length = self
                    .parse_number(15)
                    .map_err(|e| anyhow!("Error parsing subpacket_length: {}", e))?;
                let current_offset = self.offset;
                let mut subpackets = Vec::new();
                while self.offset < (current_offset + subpacket_length as usize) {
                    subpackets.push(
                        self.parse_packet_helper()
                            .map_err(|e| anyhow!("Error parsing subpacket: {}", e))?,
                    )
                }
                Ok(Box::new(OperatorBITSPacket {
                    version,
                    type_id,
                    _length_type_id: length_type_id as u8,
                    _length: subpacket_length,
                    subpackets,
                }))
            }
        }
    }

    fn parse_number(&mut self, length: usize) -> Result<u32> {
        if self.offset + length > self.length {
            bail!(
                "Reached end of message while reading number of length {}",
                length
            )
        }
        let number = (0..length)
            .map(|i| self.bits[self.offset + i])
            .fold(0, |acc, x| 2 * acc + x as u32);
        self.offset += length;
        Ok(number)
    }

    fn parse_value(&mut self) -> Result<u128> {
        let mut cont = true;
        let mut value = 0;
        while cont {
            cont = self.next()?;
            value *= 16;
            value += self
                .parse_number(4)
                .map_err(|e| anyhow!("Error parsing value: {}", e))? as u128;
        }
        Ok(value)
    }

    fn next(&mut self) -> Result<bool> {
        if self.offset + 1 > self.length {
            bail!("Reached end of message while reading bool")
        }
        let next = self.bits[self.offset];
        self.offset += 1;
        Ok(next)
    }
}

fn main() -> Result<()> {
    let inputs = include_str!("../input").trim();
    let inputs = hex::decode(inputs)?;
    let message = BitVec::from_bytes(inputs.as_slice());
    let parser = BITSParser::from(message);
    let packet = parser.parse_packet()?;

    println!(
        "The sum of the version numbers is {:?}",
        packet.version_sum()
    );
    println!("The packet evaluates to {:?}", packet.evaluate());

    Ok(())
}
