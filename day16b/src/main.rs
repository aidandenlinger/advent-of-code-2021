// Type IDs
const LITERAL_TYPE: &str = "100";

const SUM_TYPE: &str = "000";
const PRODUCT_TYPE: &str = "001";
const MIN_TYPE: &str = "010";
const MAX_TYPE: &str = "011";
const GT_TYPE: &str = "101";
const LT_TYPE: &str = "110";
const EQ_TYPE: &str = "111";

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

/// Given puzzle input, parse into a packet and return the result of packet
/// evaluation.
fn run(s: &str) -> u64 {
    let (p, _) = Packet::new(&parse(s));

    p.eval()
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

    // Return the evaluation of this packet.
    fn eval(&self) -> u64 {
        match self {
            Packet::Literal(p) => p.eval(),
            Packet::Op(p) => p.eval(),
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

    // Evaluate the literal packet to its value.
    fn eval(&self) -> u64 {
        self.value
    }
}

/// Holds a bit string that is guaranteed to not have type ID LITERAL_TYPE.
struct OperatorString<'a>(&'a str);

#[derive(Debug, PartialEq)]
/// A BITS packet of non-literal type
struct OperatorPacket {
    version: u32,
    op: Operation,
    subpackets: Vec<Packet>,
}

#[derive(Debug, PartialEq)]
enum Operation {
    Sum,
    Product,
    Minimum,
    Maximum,
    GreaterThan,
    LessThan,
    EqTo,
}

impl OperatorPacket {
    /// Given an OperatorString, return into a parsed OperatorPacket along with
    /// data that was unread.
    fn new(s: &OperatorString) -> (OperatorPacket, String) {
        let s = s.0;

        let version =
            u32::from_str_radix(s.get(VERSION_START..=VERSION_END).unwrap(), BINARY).unwrap();

        let op = match s.get(TYPE_START..=TYPE_END).unwrap() {
            SUM_TYPE => Operation::Sum,
            PRODUCT_TYPE => Operation::Product,
            MIN_TYPE => Operation::Minimum,
            MAX_TYPE => Operation::Maximum,
            GT_TYPE => Operation::GreaterThan,
            LT_TYPE => Operation::LessThan,
            EQ_TYPE => Operation::EqTo,
            _ => unreachable!(),
        };

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
                        op,
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
                        op,
                        subpackets,
                    },
                    remaining_data,
                )
            }
            _ => unreachable!(),
        }
    }

    // Return the result of evaluation.
    fn eval(&self) -> u64 {
        let operand_iter = self.subpackets.iter().map(|p| p.eval());

        match self.op {
            Operation::Sum => operand_iter.sum(),
            Operation::Product => operand_iter.product(),
            Operation::Minimum => operand_iter.min().unwrap(),
            Operation::Maximum => operand_iter.max().unwrap(),
            Operation::GreaterThan => {
                let operands = operand_iter.collect::<Vec<_>>();
                assert_eq!(2, operands.len());

                if operands.get(0).unwrap() > operands.get(1).unwrap() {
                    1
                } else {
                    0
                }
            }
            Operation::LessThan => {
                let operands = operand_iter.collect::<Vec<_>>();
                assert_eq!(2, operands.len());

                if operands.get(0).unwrap() < operands.get(1).unwrap() {
                    1
                } else {
                    0
                }
            }
            Operation::EqTo => {
                let operands = operand_iter.collect::<Vec<_>>();
                assert_eq!(2, operands.len());

                if operands.get(0).unwrap() == operands.get(1).unwrap() {
                    1
                } else {
                    0
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLES: [(&str, u64); 8] = [
        ("C200B40A82", 3),
        ("04005AC33890", 54),
        ("880086C3E88112", 7),
        ("CE00C43D881120", 9),
        ("D8005AC2A8F0", 1),
        ("F600BC2D8F", 0),
        ("9C005AC2F8F0", 0),
        ("9C0141080250320F1802104A08", 1),
    ];

    #[test]
    fn examples() {
        for (input, ans) in EXAMPLES {
            assert_eq!(ans, run(input));
        }
    }
}
