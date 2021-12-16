use bitreader::BitReader;
use hex::decode;

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    dbg!(puzzle_1(&input));
    dbg!(puzzle_2(&input));
}

fn puzzle_1(input: &str) -> u64 {
    Packet::from_hex(input).sum_version()
}

fn puzzle_2(input: &str) -> u64 {
    Packet::from_hex(input).execute()
}

#[derive(Debug, Clone, Copy)]
enum PacketLength {
    Bit(u32),
    Packet(u32),
    Value,
}
#[derive(Debug)]
struct Packet {
    version: u8,
    r#type: u8,
    value: Vec<u8>,
    length: PacketLength,
    children: Vec<Packet>,
}

impl Packet {
    fn execute(&self) -> u64 {
        match self.r#type {
            0 => self
                .children
                .iter()
                .fold(0, |acc, curr| acc + curr.execute()),
            1 => self.children.iter().map(|c| c.execute()).product(),
            2 => self
                .children
                .iter()
                .map(|c| c.execute())
                .reduce(Ord::min)
                .unwrap(),
            3 => self
                .children
                .iter()
                .map(|c| c.execute())
                .reduce(Ord::max)
                .unwrap(),
            4 => self.get_value(),
            5 => {
                if self.children[0].execute() > self.children[1].execute() {
                    1
                } else {
                    0
                }
            }
            6 => {
                if self.children[0].execute() < self.children[1].execute() {
                    1
                } else {
                    0
                }
            }
            7 => {
                if self.children[0].execute() == self.children[1].execute() {
                    1
                } else {
                    0
                }
            }
            n => panic!("{} is not a valid command", n),
        }
    }

    fn get_value(&self) -> u64 {
        self.value
            .iter()
            .fold(0u64, |acc, &curr| acc * 16 + curr as u64)
    }

    fn sum_version(&self) -> u64 {
        self.children
            .iter()
            .fold(self.version as u64, |acc, node| acc + node.sum_version())
    }
    fn from_hex(hex_str: &str) -> Self {
        let buf = decode(hex_str).unwrap();
        let mut reader = BitReader::new(&buf);
        Self::from_reader(&mut reader).unwrap()
    }
    fn from_reader(reader: &mut BitReader) -> Result<Packet, &'static str> {
        if reader.remaining() < 8 {
            println!(
                "Remaining bits: {:b}",
                reader.read_u16(reader.remaining() as u8).unwrap()
            );
            return Err("Not Enough data left");
        }
        let version = reader.read_u8(3).expect("Fail to read packet version");
        let r#type = reader.read_u8(3).expect("Fail to read packet type");
        let value = Vec::<u8>::new();
        let length = match r#type {
            4 => PacketLength::Value,
            _ => {
                let is_length_by_packet =
                    reader.read_bool().expect("Fail to read packet length type");
                if is_length_by_packet {
                    PacketLength::Packet(reader.read_u32(11).expect("Fail to reade packet length"))
                } else {
                    PacketLength::Bit(reader.read_u32(15).expect("Fail to reade packet length"))
                }
            }
        };
        let mut packet = Packet {
            version,
            r#type,
            value,
            length,
            children: Vec::new(),
        };
        match length {
            PacketLength::Bit(n) => {
                let bit_start = reader.remaining();
                loop {
                    let bits_left = bit_start - reader.remaining();
                    if bits_left == n as u64 {
                        break;
                    } else if bits_left > n as u64 {
                        panic!("Reader Overshoot!!");
                    }
                    match Packet::from_reader(reader) {
                        Ok(p) => packet.children.push(p),
                        _ => break,
                    }
                }
            }
            PacketLength::Packet(n) => {
                for _ in 0..n {
                    if let Ok(p) = Packet::from_reader(reader) {
                        packet.children.push(p);
                    }
                }
            }
            PacketLength::Value => loop {
                let is_last = reader
                    .read_bool()
                    .expect("Error parsing value read first digit");
                let digit = reader.read_u8(4).expect("Error parsing value body");
                packet.value.push(digit);
                if !is_last {
                    break;
                }
            },
        }
        Ok(packet)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_literal() {
        let packet = Packet::from_hex("D2FE28");
        dbg!(&packet);
        assert_eq!(packet.sum_version(), 6);
        assert_eq!(packet.get_value(), 2021);
    }

    #[test]
    fn two_sub_packets() {
        let packet = Packet::from_hex("38006F45291200");
        assert_eq!(packet.children[0].get_value(), 10);
        assert_eq!(packet.children[1].get_value(), 20);
        assert_eq!(packet.sum_version(), 9);
    }

    #[test]
    fn another_example_3_sub_packets() {
        let packet = Packet::from_hex("EE00D40C823060");
        assert_eq!(packet.children.len(), 3);
    }

    #[test]
    fn puzzle_1_sum_31_test() {
        let packet = Packet::from_hex("A0016C880162017C3686B18A3D4780");
        assert_eq!(packet.sum_version(), 31);
    }

    #[test]
    fn puzzle_2_sum_test() {
        let packet = Packet::from_hex("C200B40A82");
        assert_eq!(packet.execute(), 3);
    }

    #[test]
    fn puzzle_2_advanced() {
        let packet = Packet::from_hex("9C0141080250320F1802104A08");
        assert_eq!(packet.execute(), 1);
    }
}
