use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

fn main() {
    println!("{}", run(include_str!("../input.txt")));
}

/// Given puzzle input, return the lowest total risk of any path from top left
/// to bottom right.
fn run(s: &str) -> u32 {
    dijkstra(&parse(s))
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug)]
struct DistEntry {
    dist: u32,
    point: (usize, usize),
}

/// Given a grid of numbers, return the cost of the lowest cost path from the
/// top right to the bottom left.
fn dijkstra(g: &[Vec<u32>]) -> u32 {
    let start = (0, 0);
    let end = (g.get(0).unwrap().len() - 1, g.len() - 1);

    let mut dist = HashMap::from([(start, 0)]);
    let mut prev = HashMap::new();
    let mut q = BinaryHeap::from([Reverse(DistEntry {
        dist: 0,
        point: start,
    })]);

    while !q.is_empty() {
        let p = q.pop().unwrap().0;

        // We popped the end point off the queue! Return the distance
        if p.point == end {
            return p.dist;
        }

        for n @ (x, y) in neighbors(g, p.point) {
            let new_dist = p.dist + g[y][x];

            if !dist.contains_key(&n) || new_dist < *dist.get(&n).unwrap() {
                prev.insert(n, p.point);
                dist.insert(n, new_dist);
                q.push(Reverse(DistEntry {
                    dist: new_dist,
                    point: n,
                }))
            }
        }
    }

    unreachable!()
}

/// Given a grid and point, return all possible neighbors.
/// Perform various BS to switch between int types. I apologize
fn neighbors(g: &[Vec<u32>], (x, y): (usize, usize)) -> Vec<(usize, usize)> {
    fn try_point((x, y): &(i32, i32)) -> Option<(usize, usize)> {
        if x.is_negative() || y.is_negative() {
            return None;
        }

        Some(((*x).try_into().unwrap(), (*y).try_into().unwrap()))
    }

    let (x, y): (i32, i32) = (x.try_into().unwrap(), y.try_into().unwrap());
    // Filter list of all adjacent neighbors to those actually in the grid
    [(x - 1, y), (x, y - 1), (x, y + 1), (x + 1, y)]
        .iter()
        .map(try_point)
        .flatten()
        .filter(|(x, y)| g.get(*y).and_then(|gg| gg.get(*x)).is_some())
        .collect()
}

/// Given a string of a grid of numbers, return a vector representing the grid.
fn parse(s: &str) -> Vec<Vec<u32>> {
    s.lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "\
1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";

    #[test]
    fn sample_test() {
        assert_eq!(40, run(SAMPLE_INPUT));
    }
}
