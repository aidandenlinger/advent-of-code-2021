use std::collections::HashMap;

const NUM_OF_STEPS: i32 = 10;

fn main() {
    println!("{}", run(include_str!("../input.txt")));
}

/// Given puzzle input, simulate NUM_OF_STEPS steps and take the quantity of the
/// most common element, subtracting the quantity of the least common element.
fn run(s: &str) -> i32 {
    let (start, rules) = parse(s);

    let out = steps(start, &rules, NUM_OF_STEPS);
    let (min, max) = min_max(out.chars());

    max - min
}

/// Given an iterator, find the counts of the element that occurs the most and
/// the one that occurs the least.
fn min_max<I, T>(s: I) -> (i32, i32)
where
    I: Iterator<Item = T>,
    T: std::cmp::Eq + std::hash::Hash,
{
    let mut h = HashMap::new();

    for e in s {
        *h.entry(e).or_default() += 1;
    }

    (*h.values().min().unwrap(), *h.values().max().unwrap())
}

/// Perform i steps on s.
fn steps(s: &str, key: &HashMap<&str, &str>, i: i32) -> String {
    (0..i).fold(s.to_string(), |c, _| step(&c, key))
}

/// Perform one step on s.
fn step(s: &str, k: &HashMap<&str, &str>) -> String {
    let mut ans = String::from(s.chars().next().unwrap());

    // On every pair, we add the inbetween character and finish the pair.
    // We start with just the first character of every pair, so this closes
    for pair in s
        .chars()
        .collect::<Vec<_>>()
        .windows(2)
        .map(|c| c.iter().collect::<String>())
    {
        ans.push_str(k.get(pair.as_str()).unwrap());
        ans.push(pair.chars().nth(1).unwrap());
    }

    ans
}

// Given puzzle input, return the initial polymer and a HashMap of pair
// replacement rules.
fn parse(s: &str) -> (&str, HashMap<&str, &str>) {
    let (poly, rules) = s.split_once("\n\n").unwrap();

    let rules_map = HashMap::from_iter(rules.lines().map(|s| s.split_once(" -> ").unwrap()));

    (poly, rules_map)
}

#[cfg(test)]
mod tests {

    use super::*;

    const SAMPLE_INPUT: &str = "\
NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";

    #[test]
    fn sample_test() {
        assert_eq!(1588, run(SAMPLE_INPUT));
    }

    #[test]
    fn sample_steps() {
        let (start, key) = parse(SAMPLE_INPUT);
        let one = step(start, &key);
        assert_eq!("NCNBCHB", one);

        let two = step(&one, &key);
        assert_eq!("NBCCNBBBCBHCB", two);

        let three = step(&two, &key);
        assert_eq!("NBBBCNCCNBBNBNBBCHBHHBCHB", three);

        let four = step(&three, &key);
        assert_eq!("NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB", four);

        assert_eq!(
            "NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB",
            steps(start, &key, 4)
        );
    }
}
