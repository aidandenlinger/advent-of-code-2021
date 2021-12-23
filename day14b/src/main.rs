use std::collections::HashMap;

const NUM_OF_STEPS: i32 = 40;

fn main() {
    println!("{}", run(include_str!("../input.txt")));
}

/// Given puzzle input, simulate NUM_OF_STEPS steps and take the quantity of the
/// most common element, subtracting the quantity of the least common element.
fn run(s: &str) -> u64 {
    let p = Parsed::new(s);

    let out = steps(p.start, &p.key, NUM_OF_STEPS);
    let char_counts = to_char_count(&out, &p.start_end);

    char_counts.values().max().unwrap() - char_counts.values().min().unwrap()
}

/// Given a hashmap of string pairs and their counts, map to the counts of
/// individual characters.
fn to_char_count(
    m: &HashMap<String, u64>,
    (first_char, last_char): &(char, char),
) -> HashMap<char, u64> {
    // First, for each pair, add up the total counts for each unique char in
    // that pair
    // Each middle element is double counted as the start and end of a pair
    // The first and last elements are not double counted
    // Subtract 1 from the first and last element to temporarily remove them
    // Divide entire map by 2
    // then add our first and last char back

    // ex ABCAB
    //   -> "AB" : 2, "BC": 1, "CA" : 1
    // When we take the counts from those pairs, we initially get
    //   -> "A": 3, "B": 3, "C": 2
    // Now subtract 1 from A and 1 from B, to temporarily remove start/end
    //   -> "A": 2. "B": 2. "C" : 2
    // Divide all by 2 to eliminate double counted middle vars
    //   -> "A": 1, "B": 1, "C": 1
    // Add 1 back to "A" and "B" for the start and end chars:
    //  -> "A": 2, "B": 2, "C": 1

    // Count total number of pairs each char is in
    let mut out = HashMap::new();
    for (pair, count) in m {
        let (first, second) = match pair.chars().collect::<Vec<_>>()[..] {
            [f, s] => (f, s),
            _ => unreachable!(),
        };

        *out.entry(first).or_default() += count;
        *out.entry(second).or_default() += count;
    }

    // Subtract one from first/last chars
    out.entry(*first_char).and_modify(|e| *e -= 1);
    out.entry(*last_char).and_modify(|e| *e -= 1);

    // Divide all by 2 to prevent double counting
    for (_, val) in out.iter_mut() {
        *val /= 2;
    }

    // Add one back to first/last chars
    out.entry(*first_char).and_modify(|e| *e += 1);
    out.entry(*last_char).and_modify(|e| *e += 1);

    out
}

/// Given an iterator, find the counts of the element that occurs the most and
/// the one that occurs the least.
fn element_count<I, T>(s: I) -> HashMap<T, u64>
where
    I: IntoIterator<Item = T>,
    T: std::cmp::Eq + std::hash::Hash,
{
    let mut h = HashMap::new();

    for e in s {
        *h.entry(e).or_default() += 1;
    }

    h
}

/// Given a string, return all windows of size 2 as strings in a vector.
fn to_str_pair_vec(s: &str) -> Vec<String> {
    s.chars()
        .collect::<Vec<_>>()
        .windows(2)
        .map(|c| c.iter().collect::<String>())
        .collect()
}

/// Perform i steps on s.
fn steps(pairs: HashMap<String, u64>, key: &HashMap<&str, &str>, i: i32) -> HashMap<String, u64> {
    (0..i).fold(pairs, |c, _| step(c, key))
}

/// Perform one step on s.
fn step(pairs: HashMap<String, u64>, key: &HashMap<&str, &str>) -> HashMap<String, u64> {
    let mut ans = HashMap::new();

    for (pair, count) in pairs {
        let insert = key.get(pair.as_str()).unwrap();

        // For some pair AB, when we take a step:
        // AB -> A(insert char) and (insert char)B, each with the same counts as the number of ABs
        // Generate those two new pairs and add the counts of AB to them
        let new_pair_1 = pair.chars().next().unwrap().to_string() + insert;
        *ans.entry(new_pair_1).or_default() += count;

        let new_pair_2 = insert.to_string() + &pair.chars().nth(1).unwrap().to_string();
        *ans.entry(new_pair_2).or_default() += count;
    }

    ans
}

// Hold all the data generated when parsing the input string
struct Parsed<'a> {
    start: HashMap<String, u64>,
    key: HashMap<&'a str, &'a str>,
    start_end: (char, char),
}

impl Parsed<'_> {
    // Given puzzle input, return the initial polymer and a HashMap of pair
    // replacement rules.
    fn new(s: &str) -> Parsed {
        let (start, key) = s.split_once("\n\n").unwrap();

        let (first, last) = (start.chars().next().unwrap(), start.chars().last().unwrap());

        let start = element_count(to_str_pair_vec(start));

        let key = HashMap::from_iter(key.lines().map(|s| s.split_once(" -> ").unwrap()));

        Parsed {
            start,
            key,
            start_end: (first, last),
        }
    }
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
        assert_eq!(2188189693529, run(SAMPLE_INPUT));
    }

    #[test]
    fn sample_step() {
        let p = Parsed::new(SAMPLE_INPUT);
        let one = step(p.start, &p.key);
        assert_eq!(element_count(to_str_pair_vec("NCNBCHB")), one);

        let two = step(one, &p.key);
        assert_eq!(element_count(to_str_pair_vec("NBCCNBBBCBHCB")), two);

        let three = step(two, &p.key);
        assert_eq!(
            element_count(to_str_pair_vec("NBBBCNCCNBBNBNBBCHBHHBCHB")),
            three
        );

        let four = step(three, &p.key);
        assert_eq!(
            element_count(to_str_pair_vec(
                "NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB"
            )),
            four
        );
    }

    #[test]
    fn sample_steps() {
        let p = Parsed::new(SAMPLE_INPUT);
        assert_eq!(
            element_count(to_str_pair_vec(
                "NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB"
            )),
            steps(p.start, &p.key, 4)
        );
    }
}
