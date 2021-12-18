use std::collections::{BinaryHeap, VecDeque};

fn main() {
    println!("{}", run(include_str!("../input.txt")));
}

/// Given puzzle input, multiply the size of the largest three areas.
fn run(s: &str) -> i32 {
    let mut g = parse(s);
    let mut heap = BinaryHeap::new();
    let (y_max, x_max) = (g.len(), g[0].len());

    for x in 0..x_max {
        for y in 0..y_max {
            if g[y][x] == Spot::NotSearched {
                let size = bfs(&mut g, (x, y));
                heap.push(size);
            }
        }
    }

    heap.pop().unwrap() * heap.pop().unwrap() * heap.pop().unwrap()
}

#[derive(PartialEq)]
enum Spot {
    Wall,
    Searched,
    NotSearched,
}

/// Given puzzle input, return a grid of Spots
fn parse(s: &str) -> Vec<Vec<Spot>> {
    s.lines()
        .map(|l| {
            l.chars()
                .map(|i| match i.to_digit(10).unwrap() {
                    9 => Spot::Wall,
                    _ => Spot::NotSearched,
                })
                .collect()
        })
        .collect()
}

/// Run BFS on a grid of Spots starting at p, returning the size of the area
fn bfs(map: &mut [Vec<Spot>], p: (usize, usize)) -> i32 {
    let (y_max, x_max) = (map.len() - 1, map[0].len() - 1);

    // returns location of valid neighbors.
    let get_neighbors = |(x, y)| {
        let mut n = Vec::new();
        if x > 0 {
            n.push((x - 1, y));
        };
        if x < x_max {
            n.push((x + 1, y));
        };
        if y > 0 {
            n.push((x, y - 1));
        }
        if y < y_max {
            n.push((x, y + 1));
        }
        n
    };

    // init bfs
    let mut queue = VecDeque::from([p]);
    map[p.1][p.0] = Spot::Searched;
    let mut size = 1;

    while !queue.is_empty() {
        let p = queue.pop_front().unwrap();

        let neighbors = get_neighbors(p)
            .iter()
            .filter(|(x, y)| map[*y][*x] == Spot::NotSearched)
            .cloned()
            .collect::<Vec<_>>();

        // Add unsearched neighbors to the queue and mark as searched
        for n in neighbors {
            map[n.1][n.0] = Spot::Searched;
            size += 1;
            queue.push_back(n);
        }
    }

    size
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
        assert_eq!(1134, run(WEB_INPUT));
    }

    #[test]
    fn bfs_test() {
        let mut g = parse(WEB_INPUT);

        assert_eq!(3, bfs(&mut g, (0, 0)));
        assert_eq!(9, bfs(&mut g, (5, 0)));
        assert_eq!(14, bfs(&mut g, (2, 1)));
        assert_eq!(9, bfs(&mut g, (7, 2)));
    }
}
