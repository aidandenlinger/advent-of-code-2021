use std::fs;

fn main() {
    let s = fs::read_to_string("input.txt").unwrap();

    println!("{}", get_data(&s, true) * get_data(&s, false));
}

/// Get the u32 for oxygen or c02 levels
fn get_data(s: &str, oxygen: bool) -> u32 {
    let mut results: Vec<&str> = s.lines().collect();

    for i in 0.. {
        if results.len() == 1 {
            break;
        };

        let match_char = get_match_char(&results, i, oxygen);

        results = results
            .into_iter()
            .filter(|s| s.chars().nth(i).unwrap() == match_char)
            .collect();
    }

    bstring_to_u32(results[0])
}

/// Get the most/least frequent char at position i
/// Tiebreakers on most return 1, tiebreakers on least return 0
fn get_match_char(v: &Vec<&str>, pos: usize, most: bool) -> char {
    let count_1: usize = v
        .iter()
        .map(|s| s.chars().nth(pos).unwrap())
        .filter(|c| *c == '1')
        .count();
    let count_0 = v.len() - count_1;

    if most {
        if count_1 >= count_0 {
            '1'
        } else {
            '0'
        }
    } else {
        if count_0 <= count_1 {
            '0'
        } else {
            '1'
        }
    }
}

/// Convert a binary string into a u32
fn bstring_to_u32(s: &str) -> u32 {
    u32::from_str_radix(s, 2).unwrap()
}

#[cfg(test)]
mod tests {
    const S: &str = "\
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

    #[test]
    fn oxygen() {
        use super::*;
        let oxygen = get_data(&S, true);
        assert_eq!(23, oxygen);
    }

    #[test]
    fn co2() {
        use super::*;
        let co2 = get_data(&S, false);
        assert_eq!(10, co2);
    }
}
