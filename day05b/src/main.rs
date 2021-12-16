use std::cmp::{max, min};

const GRID_SIZE: usize = 1000;

fn main() {
    let lines = parse_to_lines(include_str!("../input.txt"));
    let mut g = Grid::new();
    g.draw_lines(&lines);
    println!("{}", g.two_line_overlap());
}

/// Takes puzzle input and return vector of Line
fn parse_to_lines(s: &str) -> Vec<Line> {
    s.lines()
        .map(|l| Line::new(&l.replace(" -> ", ",")))
        .collect()
}

struct Grid {
    grid: [[u8; GRID_SIZE]; GRID_SIZE],
}

impl Grid {
    // New blank grid
    fn new() -> Grid {
        Grid {
            grid: [[0; GRID_SIZE]; GRID_SIZE],
        }
    }

    // New grid with all lines drawn on it
    fn draw_lines(&mut self, lines: &[Line]) {
        for line in lines {
            self.draw_line(line)
        }
    }

    // Draw line on grid
    fn draw_line(&mut self, l: &Line) {
        if l.x1 == l.x2 {
            // Horizontal line case: x1 == x2
            let x = l.x1;
            let (y1, y2) = (min(l.y1, l.y2), max(l.y1, l.y2));
            for y in y1..=y2 {
                self.grid[y][x] += 1
            }
        } else if l.y1 == l.y2 {
            // Vertical line case: y1 == y2
            let y = l.y1;
            let (x1, x2) = (min(l.x1, l.x2), max(l.x1, l.x2));
            for x in x1..=x2 {
                self.grid[y][x] += 1
            }
        } else {
            // diagonal line! Zip from x/y1 to x/y2
            for (x, y) in range_inclusive(l.x1, l.x2).zip(range_inclusive(l.y1, l.y2)) {
                self.grid[y][x] += 1;
            }
        }
    }

    // Get the number of points that have two or more lines overlapping
    fn two_line_overlap(&self) -> usize {
        self.grid.iter().flatten().filter(|&x| x > &1).count()
    }
}

/// From https://stackoverflow.com/questions/70329833/rust-range-where-start-end
/// Wish there was a better way to get a range with negative steps
fn range_inclusive(a: usize, b: usize) -> impl Iterator<Item = usize> {
    let x: Box<dyn Iterator<Item = usize>>;
    if b > a {
        x = Box::new(a..=b)
    } else {
        x = Box::new((b..=a).rev())
    }
    x
}

#[derive(Debug, PartialEq)]
struct Line {
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
}

impl Line {
    /// Takes a comma separated string of 4 numbers and converts it into a Line
    fn new(s: &str) -> Line {
        match s.split(',').map(|n| n.parse().unwrap()).collect::<Vec<_>>()[..] {
            [x1, y1, x2, y2] => Line { x1, y1, x2, y2 },
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const WEB_INPUT: &str = "\
0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

    const BASIC: &str = "\
0,0 -> 0,1
2,2 -> 3,2";

    #[test]
    fn web_example() {
        let lines = parse_to_lines(WEB_INPUT);
        let mut g = Grid::new();
        g.draw_lines(&lines);
        assert_eq!(12, g.two_line_overlap());
    }

    #[test]
    fn draw_web() {
        let lines = parse_to_lines(WEB_INPUT);
        let mut g = Grid::new();
        g.draw_lines(&lines);

        let ans = [
            [1, 0, 1, 0, 0, 0, 0, 1, 1, 0],
            [0, 1, 1, 1, 0, 0, 0, 2, 0, 0],
            [0, 0, 2, 0, 1, 0, 1, 1, 1, 0],
            [0, 0, 0, 1, 0, 2, 0, 2, 0, 0],
            [0, 1, 1, 2, 3, 1, 3, 2, 1, 1],
            [0, 0, 0, 1, 0, 2, 0, 0, 0, 0],
            [0, 0, 1, 0, 0, 0, 1, 0, 0, 0],
            [0, 1, 0, 0, 0, 0, 0, 1, 0, 0],
            [1, 0, 0, 0, 0, 0, 0, 0, 1, 0],
            [2, 2, 2, 1, 1, 1, 0, 0, 0, 0],
        ];

        assert!(ans
            .iter()
            .flatten()
            .zip(g.grid.iter().map(|a| a.iter().take(10)).flatten())
            .all(|(x, y)| x == y));
    }

    #[test]
    fn basic_parse() {
        let ans = vec![
            Line {
                x1: 0,
                y1: 0,
                x2: 0,
                y2: 1,
            },
            Line {
                x1: 2,
                y1: 2,
                x2: 3,
                y2: 2,
            },
        ];

        assert_eq!(ans, parse_to_lines(BASIC))
    }
}
