use std::collections::{HashMap, HashSet};
fn main() {
    println!("{}", run(include_str!("../input.txt")));
}

/// Given puzzle input of a nav subsystem, return the syntax error score
fn run(s: &str) -> i32 {
    s.lines().map(error_score).flatten().sum()
}

/// Taking one line of input, return the error score of this line (or None if there is no error).
fn error_score(s: &str) -> Option<i32> {
    let open_chars = HashSet::from(['(', '[', '{', '<']);
    let close_chars = HashMap::from([('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]);
    let error_score = HashMap::from([(')', 3), (']', 57), ('}', 1197), ('>', 25137)]);

    let mut stack = Vec::new();

    for c in s.chars() {
        if open_chars.contains(&c) {
            // Add open character to stack
            stack.push(c);
        } else if !stack.is_empty() && c == *close_chars.get(stack.last().unwrap()).unwrap() {
            // We have successfully closed the last open, remove from stack
            stack.pop();
        } else {
            // This is an improper close char, return it as our first error
            return Some(*error_score.get(&c).unwrap());
        }
    }

    // We never had an improper close, no error score
    None
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
        assert_eq!(26397, run(WEB_INPUT));
    }
}
