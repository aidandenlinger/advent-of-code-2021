// Type IDs
const LITERAL_TYPE: &str = "100";

// Indices for all packets: versions from [0,2], type ID from [3,5]
const VERSION_START: usize = 0;
const VERSION_END: usize = 2;
const TYPE_START: usize = 3;
const TYPE_END: usize = 5;

// Literal Packet decoding
const HEADER: usize = 0;
const PACKET_LENGTH: usize = 5;
const LAST_PACKET_HEADER: char = '0';

// Operator Packet decoding
const TYPE_ID: usize = 6;
// Len Type ID 0 -> num of bits
const NUM_OF_BITS: &str = "0";
const BIT_NUM_START: usize = 7;
const BIT_NUM_END: usize = 21;
// Len Type ID 1 -> num of packets
const NUM_OF_PACKETS: &str = "1";
const PACKET_NUM_START: usize = 7;
const PACKET_NUM_END: usize = 17;

// Radixes
const HEX: u32 = 16;
const BINARY: u32 = 2;

fn main() {
    println!("{}", run(include_str!("../input.txt")));
}

/// Given puzzle input, parse into a packet and return the sum of all packet
/// versions within the packet.
fn run(s: &str) -> u32 {
    let (p, _) = Packet::new(&parse(s));

    p.version_sum()
}

/// Holds a string that only consists of 0s and 1s.
struct BitString(String);

/// Given a hexadecimal string, return it as a string of bits.
fn parse(s: &str) -> BitString {
    BitString(
        s.chars()
            .map(|c| format!("{:04b}", c.to_digit(HEX).unwrap()))
            .collect(),
    )
}

#[derive(Debug, PartialEq)]
/// A BITS packet.
enum Packet {
    Literal(LiteralPacket),
    Op(OperatorPacket),
}

impl Packet {
    /// Given a BITS binary message, return as a parsed packet and data remainder
    fn new(s: &BitString) -> (Packet, String) {
        let s = &s.0;

        match s.get(TYPE_START..=TYPE_END).unwrap() {
            LITERAL_TYPE => {
                let (p, r) = LiteralPacket::new(&LiteralString(s));
                (Packet::Literal(p), r)
            }
            _ => {
                let (p, r) = OperatorPacket::new(&OperatorString(s));
                (Packet::Op(p), r)
            }
        }
    }

    // Return the sum of all the versions of every packet.
    fn version_sum(&self) -> u32 {
        match self {
            Packet::Literal(p) => p.version,
            Packet::Op(p) => p.version + p.subpackets.iter().map(|p| p.version_sum()).sum::<u32>(),
        }
    }
}

/// Hold a bit string that is guaranteed to not have type ID LITERAL_TYPE.
struct LiteralString<'a>(&'a str);

#[derive(Debug, PartialEq)]
/// A BITS packet of literal type (typeID = TYPE_LITERAL).
struct LiteralPacket {
    version: u32,
    value: u64,
}

impl LiteralPacket {
    /// Given a string of a BITS literal packet, return as a parsed LiteralPacket
    /// and remaining unread string
    fn new(s: &LiteralString) -> (LiteralPacket, String) {
        let s = s.0;

        let version =
            u32::from_str_radix(s.get(VERSION_START..=VERSION_END).unwrap(), BINARY).unwrap();

        let literal_data = s.get((TYPE_END + 1)..).unwrap().chars().collect::<Vec<_>>();
        let mut chunk_iter = literal_data.chunks(PACKET_LENGTH);

        let mut value = String::new();
        loop {
            let chunk = chunk_iter.next().unwrap();

            // Push chars from chunk, skipping header bit
            value.push_str(&chunk.iter().skip(1).collect::<String>());

            // if we just pushed the last packet, break
            if *chunk.get(HEADER).unwrap() == LAST_PACKET_HEADER {
                break;
            }
        }
        // collect the remaining unused string
        let rem = chunk_iter.flatten().collect::<String>();

        let value = u64::from_str_radix(&value, BINARY).unwrap();

        (LiteralPacket { version, value }, rem)
    }
}

/// Holds a bit string that is guaranteed to not have type ID LITERAL_TYPE.
struct OperatorString<'a>(&'a str);

#[derive(Debug, PartialEq)]
/// A BITS packet of non-literal type
struct OperatorPacket {
    version: u32,
    subpackets: Vec<Packet>,
}

impl OperatorPacket {
    /// Given an OperatorString, return into a parsed OperatorPacket along with
    /// data that was unread.
    fn new(s: &OperatorString) -> (OperatorPacket, String) {
        let s = s.0;

        let version =
            u32::from_str_radix(s.get(VERSION_START..=VERSION_END).unwrap(), BINARY).unwrap();

        let mut subpackets = Vec::new();

        match s.get(TYPE_ID..=TYPE_ID).unwrap() {
            NUM_OF_BITS => {
                let len_data = s.get(BIT_NUM_START..=BIT_NUM_END).unwrap();
                let len = usize::from_str_radix(len_data, BINARY).unwrap();

                let mut remaining_data = s.get((BIT_NUM_END + 1)..).unwrap().to_string();
                let (data, rem) = remaining_data.split_at_mut(len);
                let mut data = data.to_string();

                while !data.is_empty() {
                    // data is a BitString: it is created by splitting up the original data, which is a BitString
                    let (p, rem_data) = Packet::new(&BitString(data));
                    subpackets.push(p);
                    data = rem_data;
                }

                (
                    OperatorPacket {
                        version,
                        subpackets,
                    },
                    rem.to_string(),
                )
            }
            NUM_OF_PACKETS => {
                let packet_num_data = s.get(PACKET_NUM_START..=PACKET_NUM_END).unwrap();
                let packets_to_get = usize::from_str_radix(packet_num_data, BINARY).unwrap();
                let mut remaining_data = s.get((PACKET_NUM_END) + 1..).unwrap().to_string();

                for _ in 0..packets_to_get {
                    // remaining_data is a BitString - made from existing data which is a BitString
                    let (p, r) = Packet::new(&BitString(remaining_data));
                    subpackets.push(p);
                    remaining_data = r;
                }

                (
                    OperatorPacket {
                        version,
                        subpackets,
                    },
                    remaining_data,
                )
            }
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_LITERAL: &str = "D2FE28";
    const SAMPLE_OPERATOR_LENTYPE0: &str = "38006F45291200";
    const SAMPLE_OPERATOR_LENTYPE1: &str = "EE00D40C823060";

    const SAMPLE_VERSIONSUM_1: &str = "8A004A801A8002F478";
    const SAMPLE_VERSIONSUM_2: &str = "620080001611562C8802118E34";
    const SAMPLE_VERSIONSUM_3: &str = "C0015000016115A2E0802F182340";
    const SAMPLE_VERSIONSUM_4: &str = "A0016C880162017C3686B18A3D4780";

    #[test]
    fn versionsum_test() {
        assert_eq!(16, run(SAMPLE_VERSIONSUM_1));
        assert_eq!(12, run(SAMPLE_VERSIONSUM_2));
        assert_eq!(23, run(SAMPLE_VERSIONSUM_3));
        assert_eq!(31, run(SAMPLE_VERSIONSUM_4));
    }

    #[test]
    fn literal_test() {
        let ans = Packet::Literal(LiteralPacket {
            version: 6,
            value: 2021,
        });

        assert_eq!(
            (ans, String::from("000")),
            Packet::new(&parse(SAMPLE_LITERAL))
        );
    }

    #[test]
    fn operator_lentype0_test() {
        let ans = Packet::Op(OperatorPacket {
            version: 1,
            subpackets: vec![
                Packet::Literal(LiteralPacket {
                    version: 6,
                    value: 10,
                }),
                Packet::Literal(LiteralPacket {
                    version: 2,
                    value: 20,
                }),
            ],
        });

        assert_eq!(
            (ans, String::from("0000000")),
            Packet::new(&parse(SAMPLE_OPERATOR_LENTYPE0))
        );
    }

    #[test]
    fn operator_lentype1_test() {
        let ans = Packet::Op(OperatorPacket {
            version: 7,
            subpackets: vec![
                Packet::Literal(LiteralPacket {
                    version: 2,
                    value: 1,
                }),
                Packet::Literal(LiteralPacket {
                    version: 4,
                    value: 2,
                }),
                Packet::Literal(LiteralPacket {
                    version: 1,
                    value: 3,
                }),
            ],
        });

        assert_eq!(
            (ans, String::from("00000")),
            Packet::new(&parse(SAMPLE_OPERATOR_LENTYPE1))
        );
    }
}
