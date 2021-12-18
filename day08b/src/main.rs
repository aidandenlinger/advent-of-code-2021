use std::collections::{HashMap, HashSet};

fn main() {
    println!("{}", run(include_str!("../input.txt")));
}

/// Given puzzle input, solve the problem
fn run(s: &str) -> i32 {
    s.lines()
        .map(|line| {
            let (sets, msg) = parse(line);

            let sol = solve(&sets);

            msg.iter()
                .zip((0..msg.len().try_into().unwrap()).rev())
                .map(|(&s, i)| {
                    sol[&{
                        // alphabetize the input to get the proper key
                        // Probably should've used HashSets as the key
                        let mut chars: Vec<char> = s.chars().collect();
                        chars.sort_unstable();
                        chars.iter().collect::<String>()
                    }] * 10_i32.pow(i)
                })
                .sum::<i32>()
        })
        .sum::<i32>()
}

/// Hold HashSets of segments
#[derive(Debug, PartialEq)]
struct Sets {
    one: HashSet<char>,
    four: HashSet<char>,
    seven: HashSet<char>,
    eight: HashSet<char>,
    five: Vec<HashSet<char>>,
    six: Vec<HashSet<char>>,
}

/// Given a set, solve the puzzle by returning which alphabetical string maps to which digit
fn solve(s: &Sets) -> HashMap<String, i32> {
    let mut key: HashMap<char, char> = HashMap::new();

    // Intersection over all numbers of segment length 5: {a,d,g}
    let five_intersect = fold_intersect(&s.five);
    // Intersection over all numbers of segment length 6: {a,b,f,g}
    let six_intersect = fold_intersect(&s.six);

    // 7 {a, c, f} - 1 {c, f} = a
    key.insert('a', *s.seven.difference(&s.one).next().unwrap());

    // intersect over 5: {a,d,g} intersect intersect over 6: {a,b,f,g} = {a,g}) - {a} = g
    key.insert(
        'g',
        *five_intersect
            .intersection(&six_intersect)
            .cloned()
            .collect::<HashSet<_>>()
            .difference(&HashSet::from_iter(make_set(&['a'], &key)))
            .next()
            .unwrap(),
    );

    // intersect over 5: {a,d,g} - {a,g} = d
    key.insert(
        'd',
        *five_intersect
            .difference(&HashSet::from_iter(make_set(&['a', 'g'], &key)))
            .next()
            .unwrap(),
    );

    // 6 intersect {a,b,f,g} intersect 1 {c,f} = f
    key.insert('f', *six_intersect.intersection(&s.one).next().unwrap());

    // 1 {c,f} - {f} = c
    key.insert(
        'c',
        *s.one.difference(&make_set(&['f'], &key)).next().unwrap(),
    );

    // 6 intersect {a,b,f,g} - {a,f,g} = b
    key.insert(
        'b',
        *six_intersect
            .difference(&make_set(&['a', 'f', 'g'], &key))
            .next()
            .unwrap(),
    );

    // 8 {a,b,c,d,e,f,g} - {a,b,c,d,f,g} = e
    key.insert(
        'e',
        *s.eight
            .difference(&make_set(&['a', 'b', 'c', 'd', 'f', 'g'], &key))
            .next()
            .unwrap(),
    );

    key_to_table(&key)
}

/// Given the key, find the alphabetical string that maps to each digit
fn key_to_table(key: &HashMap<char, char>) -> HashMap<String, i32> {
    let mut table = HashMap::new();

    let alphabet_str = |v: &[char]| {
        let mut s: Vec<char> = v.iter().map(|c| key[c]).collect();
        s.sort_unstable();
        s.iter().collect()
    };

    // Insert the alphabetical string
    table.insert(alphabet_str(&['a', 'b', 'c', 'e', 'f', 'g']), 0);
    table.insert(alphabet_str(&['c', 'f']), 1);
    table.insert(alphabet_str(&['a', 'c', 'd', 'e', 'g']), 2);
    table.insert(alphabet_str(&['a', 'c', 'd', 'f', 'g']), 3);
    table.insert(alphabet_str(&['b', 'c', 'd', 'f']), 4);
    table.insert(alphabet_str(&['a', 'b', 'd', 'f', 'g']), 5);
    table.insert(alphabet_str(&['a', 'b', 'd', 'e', 'f', 'g']), 6);
    table.insert(alphabet_str(&['a', 'c', 'f']), 7);
    table.insert(alphabet_str(&['a', 'b', 'c', 'd', 'e', 'f', 'g']), 8);
    table.insert(alphabet_str(&['a', 'b', 'c', 'd', 'f', 'g']), 9);

    table
}

/// Given a char slice and a key to translate those cars, make a new set
fn make_set(c: &[char], k: &HashMap<char, char>) -> HashSet<char> {
    HashSet::from_iter(c.iter().map(|i| k[i]))
}

/// Run an intersection across a slice of HashSets (this sucks)
fn fold_intersect(v: &[HashSet<char>]) -> HashSet<char> {
    v.iter().skip(1).fold(v[0].clone(), |acc, s| {
        acc.intersection(s).cloned().collect()
    })
}

/// Take one line of puzzle input and turn it into Sets and the output message
fn parse(s: &str) -> (Sets, Vec<&str>) {
    let (patterns, msg) = s.split_once(" | ").unwrap();

    // 1 has 2 segments on, 4 has 4 segments, 7 has 3 segments, 8 has 7 segments
    let mut uniq_patterns = [2, 4, 3, 7]
        .iter()
        .map(|&i| set_from_len(patterns, i))
        .flatten();

    // There are 3 numbers with 5 segments on {2, 3, 5} and 3 numbers with 6 {0,6,9}
    let mut remaining_patterns = [5, 6].iter().map(|&i| set_from_len(patterns, i));

    (
        Sets {
            one: uniq_patterns.next().unwrap(),
            four: uniq_patterns.next().unwrap(),
            seven: uniq_patterns.next().unwrap(),
            eight: uniq_patterns.next().unwrap(),
            five: remaining_patterns.next().unwrap(),
            six: remaining_patterns.next().unwrap(),
        },
        msg.split_whitespace().collect(),
    )
}

/// Given a length, make a HashSet for each pattern with that length
fn set_from_len(s: &str, l: i32) -> Vec<HashSet<char>> {
    s.split_whitespace()
        .filter(|s| s.len() == l.try_into().unwrap())
        .map(|s| HashSet::from_iter(s.chars()))
        .collect()
}

#[cfg(test)]
mod tests {

    use super::*;

    const WEB_SIMPLE: &str =
        "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";

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
    fn web_simple() {
        assert_eq!(5353, run(WEB_SIMPLE));
    }

    #[test]
    fn web_hard() {
        assert_eq!(61229, run(WEB_INPUT));
    }
}
