use std::collections::{HashMap, HashSet};
fn main() {
    println!("{}", run(include_str!("../input.txt")));
}

/// Given puzzle input of a nav subsystem, return the median autocorrect score
fn run(s: &str) -> u64 {
    let mut scores: Vec<u64> = s.lines().map(complete).flatten().collect();
    scores.sort_unstable();
    *scores.get(scores.len() / 2).unwrap()
}

/// Given one line of input, return its autocomplete score, or None if line is corrupted
fn complete(s: &str) -> Option<u64> {
    let open_chars = HashSet::from(['(', '[', '{', '<']);
    let close_chars = HashMap::from([('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]);
    let autocomplete_score = HashMap::from([(')', 1), (']', 2), ('}', 3), ('>', 4)]);

    let mut stack = Vec::new();

    for c in s.chars() {
        if open_chars.contains(&c) {
            // Add open character to stack
            stack.push(c);
        } else if !stack.is_empty() && c == *close_chars.get(stack.last().unwrap()).unwrap() {
            // We have successfully closed the last open, remove from stack
            stack.pop();
        } else {
            // This line is corrupted
            return None;
        }
    }

    // Run backwards through the stack, map to close tokens, fold for autocorrect score
    Some(
        stack
            .iter()
            .rev()
            .map(|c| close_chars.get(c).unwrap())
            .map(|c| autocomplete_score.get(c).unwrap())
            .fold(0, |acc, i| acc * 5 + i),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const WEB_INPUT: &str = "\
[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

    #[test]
    fn web_example() {
        assert_eq!(288957, run(WEB_INPUT));
    }
}
