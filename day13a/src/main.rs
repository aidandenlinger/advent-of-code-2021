fn main() {
    println!("{}", run(include_str!("../input.txt")));
}

/// Given puzzle input, return how many dots are visible after the first fold
/// instruction.
fn run(s: &str) -> u32 {
    let (points, instrs) = parse(s);

    let points = execute_instruction(&points, instrs.get(0).unwrap());

    points.len().try_into().unwrap()
}

/// Given a list of dotted points and an instruction, return a new grid with the
/// dotted points after executing the instruction.
fn execute_instruction(v: &[(u32, u32)], i: &Instr) -> Vec<(u32, u32)> {
    let (mut l, r): (Vec<_>, Vec<_>) = v.iter().partition(|(x, y)| match i {
        Instr::X(n) => x < n,
        Instr::Y(n) => y < n,
    });

    // Filter out the points ON line n, as we don't want them
    let r = r
        .iter()
        .filter(|(x, y)| match i {
            Instr::X(n) => x != n,
            Instr::Y(n) => y != n,
        })
        .cloned()
        .collect::<Vec<_>>();

    // Now we map r's points to l's dimensions
    let r_translate: Vec<(u32, u32)> = r
        .iter()
        .map(|(x, y)| match i {
            // Take the distance from the fold, multiply it by 2 (to get to the
            // fold line, and then shift FROM the fold line), then shift that
            // much left or up
            Instr::X(n) => (*x - ((*x - n) * 2), *y),
            Instr::Y(n) => (*x, *y - ((*y - n) * 2)),
        })
        .collect::<Vec<_>>();

    for p in r_translate {
        if !l.contains(&p) {
            l.push(p);
        }
    }

    l
}

/// Holds a fold on x or a fold on y instruction.
enum Instr {
    X(u32),
    Y(u32),
}

/// Given puzzle input, return a vector of points with dots and a vector of instructions.
fn parse(s: &str) -> (Vec<(u32, u32)>, Vec<Instr>) {
    // Split at empty newline
    let (points, instrs) = s.split_once("\n\n").unwrap();

    let points = points
        .lines()
        .map(|s| {
            s.split(',')
                .map(|n| n.parse().unwrap())
                .collect::<Vec<u32>>()
        })
        .map(|v| (*v.get(0).unwrap(), *v.get(1).unwrap()))
        .collect();

    let instrs = instrs
        .lines()
        .map(|s| {
            match s
                .split_whitespace()
                .nth(2)
                .unwrap()
                .split_once('=')
                .unwrap()
            {
                ("x", i) => Instr::X(i.parse().unwrap()),
                ("y", i) => Instr::Y(i.parse().unwrap()),
                _ => unreachable!(),
            }
        })
        .collect();

    (points, instrs)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "\
6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";

    #[test]
    fn sample_test() {
        assert_eq!(17, run(SAMPLE_INPUT));
    }

    #[test]
    fn sample_test_two_steps() {
        let (points, instrs) = parse(SAMPLE_INPUT);

        // perform BOTH steps to ensure we handle X and Y case correctly
        let points = execute_instruction(&points, instrs.get(0).unwrap());
        let points = execute_instruction(&points, instrs.get(1).unwrap());

        assert_eq!(16, points.len())
    }
}
