use std::fs;

fn main() {
    let s = fs::read_to_string("input.txt").unwrap();

    let counts = get_counts(&s);

    let gamma = counts.iter().map(|c| c.max()).collect::<String>();
    let epsilon = flip(&gamma);

    let gamma = string_to_int(gamma);
    let epsilon = string_to_int(epsilon);

    println!("{}", gamma * epsilon)
}

fn string_to_int(s: String) -> u32 {
    u32::from_str_radix(&s, 2).unwrap()
}

/// Flip a binary string
fn flip(s: &String) -> String {
    s.chars()
        .map(|c| if c == '0' { '1' } else { '0' })
        .collect::<String>()
}

/// Given a string of binary strings, get the counts at each position
fn get_counts(s: &str) -> Vec<Count> {
    let mut counts: Vec<Count> = vec![];

    for line in s.lines() {
        for (i, c) in line.chars().enumerate() {
            // Init count if needed
            if let None = counts.get(i) {
                counts.push(Count::new())
            }
            if c == '0' {
                counts[i].add_zero()
            } else if c == '1' {
                counts[i].add_one()
            } else {
                unreachable!()
            }
        }
    }

    counts
}

#[derive(Debug)]
struct Count {
    zero: i32,
    one: i32,
}

impl Count {
    fn new() -> Count {
        Count { zero: 0, one: 0 }
    }

    fn add_zero(&mut self) {
        self.zero += 1
    }

    fn add_one(&mut self) {
        self.one += 1
    }

    fn max(&self) -> char {
        if self.zero > self.one {
            '0'
        } else if self.one > self.zero {
            '1'
        } else {
            unreachable!()
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn website_sample() {
        use super::*;
        //let s = fs::read_to_string("index.txt").unwrap();
        let s = "\
00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

        let counts = get_counts(s);

        let gamma = counts.iter().map(|c| c.max()).collect::<String>();
        let epsilon = flip(&gamma);

        let gamma = string_to_int(gamma);
        let epsilon = string_to_int(epsilon);

        assert_eq!(22, gamma);
        assert_eq!(9, epsilon);
    }
}
