fn main() {
    println!("{}", run(include_str!("../input.txt")));
}

/// Given puzzle input, find the total sum of the height of low points + 1.
fn run(s: &str) -> u32 {
    let g = parse(s);
    find_low_point_risk(&g)
}

/// Given puzzle input, return a grid of numbers
fn parse(s: &str) -> Vec<Vec<u32>> {
    s.lines()
        .map(|l| l.chars().map(|i| i.to_digit(10).unwrap()).collect())
        .collect()
}

/// Given a grid, find the risk level of its low points
fn find_low_point_risk(v: &[Vec<u32>]) -> u32 {
    let mut risk_levels = Vec::new();

    let (y_max, x_max) = (v.len(), v[0].len());

    for y in 0..y_max {
        for x in 0..x_max {
            let p = v[y][x];
            if get_neighbors(v, x, y).iter().all(|&pp| p < pp) {
                risk_levels.push(p + 1);
            }
        }
    }

    risk_levels.iter().sum()
}

/// Return the value of all applicable neighbors. Would use a range but that
/// doesn't seem like I'd really be gaining much here
fn get_neighbors(v: &[Vec<u32>], x: usize, y: usize) -> Vec<u32> {
    let (y_max, x_max) = (v.len() - 1, v[0].len() - 1);
    let mut n = Vec::new();
    if x > 0 {
        n.push(v[y][x - 1]);
    }
    if x < x_max {
        n.push(v[y][x + 1]);
    }
    if y > 0 {
        n.push(v[y - 1][x]);
    }
    if y < y_max {
        n.push(v[y + 1][x]);
    }

    n
}

#[cfg(test)]
mod tests {
    use super::*;

    // Note: this is 10x5
    const WEB_INPUT: &str = "\
2199943210
3987894921
9856789892
8767896789
9899965678";

    #[test]
    fn web_example() {
        assert_eq!(15, run(WEB_INPUT));
    }
}
