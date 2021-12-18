const UNIQ_LENS: [usize; 4] = [2, 3, 4, 7];

fn main() {
    println!(
        "{}",
        include_str!("../input.txt")
            .lines()
            .map(|s| count_1_4_7_8(&get_vec(s)))
            .sum::<i32>()
    )
}

/// Take a single puzzle input line and convert it into a vector of output strings
fn get_vec(s: &str) -> Vec<&str> {
    s.split(" | ").nth(1).unwrap().split_whitespace().collect()
}

/// Count the number of 1s, 4s, 7s, and 8s in the vector since they have unique
/// number of segments on, as stored in UNIQ_LENS
fn count_1_4_7_8(v: &[&str]) -> i32 {
    v.iter()
        .filter(|s| UNIQ_LENS.contains(&s.len()))
        .count()
        .try_into()
        .unwrap()
}

#[cfg(test)]
mod tests {

    use super::*;

    const WEB_INPUT: &str = "\
be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

    #[test]
    fn web_example() {
        assert_eq!(
            26,
            WEB_INPUT.lines().map(|s| count_1_4_7_8(&get_vec(s))).sum()
        )
    }

    #[test]
    fn web_parse() {
        let first_line = WEB_INPUT.lines().next().unwrap();

        assert_eq!(
            vec!["fdgacbe", "cefdb", "cefbgd", "gcbe"],
            get_vec(first_line)
        );
    }
}
