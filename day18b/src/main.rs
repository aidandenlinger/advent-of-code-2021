use std::cmp::max;
use std::fmt;
use std::ops::Add;

const OPEN_CHAR: char = '[';
const CLOSED_CHAR: char = ']';
const SEP_CHAR: char = ',';
const EXPLODE_DEPTH: i32 = 5;
const SPLIT_MIN: u32 = 10;

fn main() {
    println!("{}", run(include_str!("../input.txt")));
}

/// Given a shellfish homework problem, return the largest magnitude from adding
/// two of the snailfish numbers
fn run(s: &str) -> u32 {
    let list = parse(s);

    let mut max_mag = 0;

    for n1 in list.iter() {
        for n2 in list.iter().skip_while(|n2| *n2 == n1) {
            let curr_mag = n1.clone().add(n2.clone()).magnitude();

            max_mag = max(curr_mag, max_mag);
        }
    }

    max_mag
}

/// Given a file of shellfish numbers, parse them all into ReducedShellNums.
fn parse(s: &str) -> Vec<ReducedShellNum> {
    s.lines().map(|i| BasicShellNum::new(i).reduce()).collect()
}

/// The tokens in the shellfish number
#[derive(Debug, PartialEq, Clone)]
enum Elem {
    Open,
    Close,
    Sep,
    Num(u32),
}

/// Wrapper to hold a tokenized shellfish number
#[derive(Debug)]
struct BasicShellNum {
    num: Vec<Elem>,
}

#[derive(Debug, PartialEq, Clone)]
struct ReducedShellNum {
    num: Vec<Elem>,
}

impl BasicShellNum {
    /// Given a shell number, tokenize it into a ShellNum
    fn new(s: &str) -> BasicShellNum {
        let mut v = vec![];

        for c in s.chars() {
            v.push(match c {
                OPEN_CHAR => Elem::Open,
                CLOSED_CHAR => Elem::Close,
                SEP_CHAR => Elem::Sep,
                _ => Elem::Num(c.to_digit(10).unwrap()),
            })
        }

        BasicShellNum { num: v }
    }

    /// Scan for explodes that need to occur. Return false if there are no
    /// explosions needed, or true if an explosion was needed and performed.
    /// Will modify self if explode action is needed.
    fn explode(&mut self) -> bool {
        let mut pair_depth = 0;
        let mut start = None;

        for (i, item) in self.num.iter().enumerate() {
            match item {
                Elem::Open => pair_depth += 1,
                Elem::Close => pair_depth -= 1,
                _ => continue,
            }

            if pair_depth == EXPLODE_DEPTH {
                // We have officially entered the left-most pair, note it and break
                start = Some(i);
                break;
            }
        }

        if start.is_none() {
            // There is no pair to explode, return false
            return false;
        }
        let start = start.unwrap();

        // Now we extract our numbers
        // This is the layout:
        // Open  Num Sep Num  Close
        // start +1  +2  +3    +4
        let (left, right) = match self.num[start + 1..=start + 3] {
            [Elem::Num(left), _, Elem::Num(right)] => (left, right),
            _ => unreachable!(),
        };

        /// Add val to first Elem::Num encounted on the iterator
        fn inc_nearest_num<'a>(val: u32, iter: impl Iterator<Item = &'a mut Elem>) {
            for elem in iter {
                match elem {
                    Elem::Num(i) => {
                        *i += val;
                        break;
                    }
                    _ => continue,
                }
            }
        }

        inc_nearest_num(left, self.num[0..start].iter_mut().rev());
        inc_nearest_num(right, self.num[start + 5..].iter_mut());

        // Finally, replace the pair with 0
        self.num.splice(start..=start + 4, vec![Elem::Num(0)]);

        true
    }

    /// Scan for splits that need to occur. Return false if no splits needed,
    /// or true if a split took place. Will modify self if split is needed.
    fn split(&mut self) -> bool {
        match self.num.iter().enumerate().find(|(_, i)| match i {
            Elem::Num(i) => *i >= SPLIT_MIN,
            _ => false,
        }) {
            Some((i, Elem::Num(num))) => {
                let left = num / 2;
                let right = num - left;
                // Replace our num with the new pair
                self.num.splice(
                    i..=i,
                    vec![
                        Elem::Open,
                        Elem::Num(left),
                        Elem::Sep,
                        Elem::Num(right),
                        Elem::Close,
                    ],
                );
                true
            }
            _ => false,
        }
    }

    /// Follow the reduction steps to convert into a ReducedShellNum
    fn reduce(mut self) -> ReducedShellNum {
        // Keep exploding then splitting until both are false, making it reduced
        while self.explode() || self.split() {}
        ReducedShellNum { num: self.num }
    }
}

impl Add for ReducedShellNum {
    type Output = Self;

    /// Add two ReducedShellNums together, with reduction.
    fn add(self, other: Self) -> Self::Output {
        self.join(other).reduce()
    }
}

impl ReducedShellNum {
    /// Adds two shellnumbers together, but DOES NOT REDUCE THEM!
    fn join(mut self, mut other: ReducedShellNum) -> BasicShellNum {
        let mut ans = vec![Elem::Open];

        ans.append(&mut self.num);
        ans.push(Elem::Sep);
        ans.append(&mut other.num);

        ans.push(Elem::Close);

        BasicShellNum { num: ans }
    }

    /// Return the magnitude of the current ReducedShellNum.
    fn magnitude(&self) -> u32 {
        // All things must eventually become a tree.
        BinaryTree::new(self).magnitude()
    }
}

enum BinaryTree {
    Pair {
        left: Box<BinaryTree>,
        right: Box<BinaryTree>,
    },
    Num(u32),
}

impl BinaryTree {
    /// Create a BinaryTree from a ReducedShellNum.
    fn new(r: &ReducedShellNum) -> BinaryTree {
        // All of this is a bad idea! Am I really introducing a binary tree
        // just for this? Yes!
        BinaryTree::parse(&r.num)
    }

    /// Parse a Vector of Elements into a BinaryTree
    /// Should only be called by BinaryTree::new, but I'm too lazy to split this
    /// into multiple files and make this private
    fn parse(v: &[Elem]) -> BinaryTree {
        assert!(v[0] == Elem::Open);
        let mut pair_depth = 1;
        let mut sep_index = None;

        for (i, elem) in v.iter().enumerate().skip(1) {
            match elem {
                Elem::Open => pair_depth += 1,
                Elem::Close => pair_depth -= 1,
                Elem::Sep if pair_depth == 1 => {
                    sep_index = Some(i);
                    break;
                }
                _ => continue,
            }
        }

        // there WILL be a sep index
        let (left, right) = v.split_at(sep_index.unwrap());
        // Truncation
        // Remove first character of left, it's the opening for this pair
        let left = &left[1..];
        // Remove first character of right, it's the separator for this pair
        // Remove last character of right, it's the closing for this pair
        let right = &right[1..right.len() - 1];

        let define_elem = |e: &[Elem]| match e[..] {
            [Elem::Num(i)] => BinaryTree::Num(i),
            _ => BinaryTree::parse(e),
        };

        BinaryTree::Pair {
            left: Box::new(define_elem(left)),
            right: Box::new(define_elem(right)),
        }
    }

    /// Calculate the magnitude of the BinaryTree.
    fn magnitude(self) -> u32 {
        match self {
            BinaryTree::Pair { left, right } => 3 * left.magnitude() + 2 * right.magnitude(),
            BinaryTree::Num(i) => i,
        }
    }
}

/// Print a shellnum with the same formatting as the input
fn print_shell_num(v: &[Elem], f: &mut fmt::Formatter<'_>) -> fmt::Result {
    for e in v {
        write!(f, "{}", e)?;
    }

    Ok(())
}

impl fmt::Display for BasicShellNum {
    /// Print a shellnum with the same formatting as the input
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        print_shell_num(&self.num, f)
    }
}

impl fmt::Display for ReducedShellNum {
    /// Print a shellnum with the same formatting as the input
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        print_shell_num(&self.num, f)
    }
}

impl fmt::Display for Elem {
    /// Print out the element with its coresponding character.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Elem::Close => write!(f, "{}", CLOSED_CHAR),
            Elem::Open => write!(f, "{}", OPEN_CHAR),
            Elem::Sep => write!(f, "{}", SEP_CHAR),
            Elem::Num(i) => write!(f, "{}", i),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    const PARSE_EXAMPLES: &str = "\
[1,2]
[[1,2],3]
[9,[8,7]]
[[1,9],[8,5]]
[[[[1,2],[3,4]],[[5,6],[7,8]]],9]
[[[9,[3,8]],[[0,9],6]],[[[3,7],[4,9]],3]]
[[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]";

    const EXPLODE_EXAMPLES: [(&str, &str); 5] = [
        ("[[[[[9,8],1],2],3],4]", "[[[[0,9],2],3],4]"),
        ("[7,[6,[5,[4,[3,2]]]]]", "[7,[6,[5,[7,0]]]]"),
        ("[[6,[5,[4,[3,2]]]],1]", "[[6,[5,[7,0]]],3]"),
        (
            "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]",
            "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
        ),
        (
            "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
            "[[3,[2,[8,0]]],[9,[5,[7,0]]]]",
        ),
    ];

    #[test]
    fn greatest_magnitude_test() {
        assert_eq!(
            3993,
            run("\
[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]")
        )
    }

    #[test]
    fn magnitute_tests() {
        let mag_test = |ans, list| assert_eq!(ans, BasicShellNum::new(list).reduce().magnitude());

        mag_test(29, "[9,1]");
        mag_test(21, "[1,9]");
        mag_test(129, "[[9,1],[1,9]]");
        mag_test(143, "[[1,2],[[3,4],5]]");
        mag_test(1384, "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");
        mag_test(445, "[[[[1,1],[2,2]],[3,3]],[4,4]]");
        mag_test(791, "[[[[3,0],[5,3]],[4,4]],[5,5]]");
        mag_test(1137, "[[[[5,0],[7,4]],[5,5]],[6,6]]");
        mag_test(
            3488,
            "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]",
        );
    }

    #[test]
    /// Test a basic addition between two numbers
    fn basic_add() {
        let n1 = BasicShellNum::new("[[[[4,3],4],4],[7,[[8,4],9]]]").reduce();
        let n2 = BasicShellNum::new("[1,1]").reduce();

        assert_eq!(
            "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]",
            format!("{}", n1.add(n2))
        );
    }

    #[test]
    /// Test a basic split
    fn split_test() {
        let mut start = BasicShellNum::new("[[[[0,7],4],[1,[0,1]]],[1,1]]");
        // lazy hack because I don't want to change my parser to interpret multidigit numbers!
        // Fun fact: that's how my parser worked until I realized I only needed single digit numbers and simplified it!
        // Forgot about test cases! Should've just left it there!
        start.num[13] = Elem::Num(15);
        start.num[18] = Elem::Num(13);
        // So now our input num is [[[[0,7],4],[15,[0,13]]],[1,1]], I'm sorry

        start.split();

        assert_eq!("[[[[0,7],4],[[7,8],[0,13]]],[1,1]]", format!("{}", start));

        start.split();

        assert_eq!(
            "[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]",
            format!("{}", start)
        );
    }

    #[test]
    // Test explosion test cases
    fn explode_test() {
        for (start, ans) in EXPLODE_EXAMPLES {
            let mut start = BasicShellNum::new(start);

            start.explode();

            assert_eq!(ans, format!("{}", start))
        }
    }

    #[test]
    /// Confirm that we can read in a shellfish number, tokenize it, and turn it
    /// back to the same string
    fn parse_test() {
        for num in PARSE_EXAMPLES.split('\n') {
            assert_eq!(num, format!("{}", BasicShellNum::new(num)));
        }
    }

    #[test]
    /// Test joining two shell nums together.
    fn join_test() {
        let n1 = BasicShellNum::new("[1,2]").reduce();
        let n2 = BasicShellNum::new("[[3,4],5]").reduce();

        assert_eq!("[[1,2],[[3,4],5]]", format!("{}", n1.join(n2)))
    }
}
