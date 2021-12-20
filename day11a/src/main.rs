const NUM_OF_STEPS: i32 = 100;

fn main() {
    println!("{}", run(include_str!("../input.txt")));
}

/// Given a grid of dumbo octopuses, determine the number of flashes after NUM_OF_STEPS steps.
fn run(s: &str) -> u32 {
    let mut g = parse(s);

    let mut flashes = 0;
    for _ in 0..NUM_OF_STEPS {
        flashes += step(&mut g);
    }

    flashes
}

/// Given a grid, evaluate one step and return the number of flashes from that step
fn step(g: &mut [Vec<u32>]) -> u32 {
    // First, increase all octopus energy by 1
    for row in g.iter_mut() {
        for p in row {
            *p += 1;
        }
    }

    let mut flashes = 0;
    let mut flashers = gen_flashers(g);

    // These are octopi that need to flash
    while !flashers.is_empty() {
        for p in flashers.iter() {
            // We are flashing - set the octopus to 0
            g[p.1][p.0] = 0;
            flashes += 1;

            // Increase all adjacent by 1
            for a in adjacent(g, *p) {
                // octopi that have already flashed this turn are already 0
                if g[a.1][a.0] != 0 {
                    g[a.1][a.0] += 1;
                }
            }
        }

        // We have flashed all nines. Now update to see if there are more to flash
        flashers = gen_flashers(g);
    }

    flashes
}

/// Given a grid, return a list of points that need to flash
fn gen_flashers(g: &[Vec<u32>]) -> Vec<(usize, usize)> {
    let mut v = Vec::new();
    for x in 0..g[0].len() {
        for (y, col) in g.iter().enumerate() {
            if col[x] > 9 {
                v.push((x, y))
            }
        }
    }
    v
}

/// Given a grid and a point, return valid coordinates of adjacent points
fn adjacent(g: &[Vec<u32>], (x, y): (usize, usize)) -> Vec<(usize, usize)> {
    /// Safely convert i32 tuple to usize tuple
    /// Disgusting, but lets me do this easily.
    /// Feel like I'm cheating here and there's gotta be a better way to do this
    /// I just want any operation that underflows/overflows to propogate into a
    /// None which I can then flatten
    fn try_point((x, y): &(i32, i32)) -> Option<(usize, usize)> {
        if x.is_negative() || y.is_negative() {
            return None;
        }

        Some(((*x).try_into().unwrap(), (*y).try_into().unwrap()))
    }

    let (x, y): (i32, i32) = (x.try_into().unwrap(), y.try_into().unwrap());
    // Filter list of all adjacent neighbors to those actually in the grid
    [
        (x - 1, y - 1),
        (x - 1, y),
        (x - 1, y + 1),
        (x, y - 1),
        (x, y + 1),
        (x + 1, y - 1),
        (x + 1, y),
        (x + 1, y + 1),
    ]
    .iter()
    .map(try_point)
    .flatten()
    .filter(|(x, y)| g.get(*y).and_then(|gg| gg.get(*x)).is_some())
    .collect()
}

/// Given a grid of numbers in text, turn into a vector grid
fn parse(s: &str) -> Vec<Vec<u32>> {
    s.lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    const WEB_INPUT: &str = "\
5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

    #[test]
    fn web_example() {
        assert_eq!(1656, run(WEB_INPUT));
    }

    #[test]
    fn step_test() {
        let s = "\
11111
19991
19191
19991
11111";

        let mut g = parse(s);

        step(&mut g);

        let ans = [
            [3, 4, 5, 4, 3],
            [4, 0, 0, 0, 4],
            [5, 0, 0, 0, 5],
            [4, 0, 0, 0, 4],
            [3, 4, 5, 4, 3],
        ];

        assert!(ans
            .iter()
            .flatten()
            .zip(g.iter().flatten())
            .all(|(a, g)| a == g));
    }
}
