use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

const EXTEND_SIZE: usize = 5;

fn main() {
    println!("{}", run(include_str!("../input.txt")));
}

/// Given puzzle input, return the lowest total risk of any path from top left
/// to bottom right on the extended grid.
fn run(s: &str) -> u32 {
    dijkstra(&extend(&parse(s)))
}

/// Given a grid from input, extend it EXTEND_SIZE times larger, with each right or
/// downwards grid increasing all numbers by 1.
fn extend(g: &[Vec<u32>]) -> Vec<Vec<u32>> {
    // end result: chunks[n] is g incremented by n in [1,9] space.
    let chunks: Vec<Vec<Vec<u32>>> = (0..9)
        .map(|n| {
            g.iter()
                // (i + n + 8) % 9 + 1 => adding 8 in mod 9 is the same as subtracting 1 mod 9. so i + n + 8 == i + n - 1.
                // This maps i <- [1,9] to i-1 <- [0,8] with modulus. Then, add 1 to shift back to [1,9].
                // ex when n=0, i + n + 8 = i + 8 = i - 1. add 1 to return to [1,9] with no translation
                // In general, we get i + 1 + 8 = i + n - 1. then we add 1 to translate to [1,9], bringing i+n
                .map(|l| l.iter().map(|i| (((i + n + 8) % 9) + 1)).collect())
                .collect()
        })
        .collect();

    // Now we need to stitch our chunks together. The pattern is
    // 0 1 2 3 4 -> one CHUNK row
    // 1 2 3 4 5
    // 2 3 4 5 6
    // 3 4 5 6 7
    // 4 5 6 7 8

    // g is one chunk
    let rows_per_chunk = g.len();
    let nums_per_row = g.get(0).unwrap().len();

    // my frankenstein's monster
    let ans: Vec<Vec<u32>> = (0..EXTEND_SIZE)
        .map(|e| {
            // e is the CHUNK row we're currently making
            (0..rows_per_chunk)
                .map(|r| {
                    // r is the row WITHIN the chunk that we're making
                    (0..EXTEND_SIZE) // this will be the chunk indexes we're accessing
                        .map(|n| (n + e) % 9) // use e to shift our range to the correct chunk indexes
                        .map(|n| &chunks[n]) // get those chunk indexes
                        .flatten() // flatten chunks into Vec of Vecs
                        .skip(r) // now filter to the vecs we need, by moving ahead r and
                        .step_by(nums_per_row) // skipping between chunks - so now we have the r-th row of each chunk
                        .flatten() // flatten and collect the rows into one vec - one row of the final grid
                        .cloned() // since grids is a local var and we need ans to live beyond it, clone the nums
                        .collect::<Vec<u32>>() // collect into one row
                })
                .collect::<Vec<Vec<u32>>>() // collect all rows into one chunk
        })
        .flatten() // flatten chunks to be one grid
        .collect();

    ans
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

    let mut dist: HashMap<(usize, usize), u32> = HashMap::from([(start, 0)]);
    let mut prev: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
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

    const SAMPLE_EXTEND: &str = "\
11637517422274862853338597396444961841755517295286
13813736722492484783351359589446246169155735727126
21365113283247622439435873354154698446526571955763
36949315694715142671582625378269373648937148475914
74634171118574528222968563933317967414442817852555
13191281372421239248353234135946434524615754563572
13599124212461123532357223464346833457545794456865
31254216394236532741534764385264587549637569865174
12931385212314249632342535174345364628545647573965
23119445813422155692453326671356443778246755488935
22748628533385973964449618417555172952866628316397
24924847833513595894462461691557357271266846838237
32476224394358733541546984465265719557637682166874
47151426715826253782693736489371484759148259586125
85745282229685639333179674144428178525553928963666
24212392483532341359464345246157545635726865674683
24611235323572234643468334575457944568656815567976
42365327415347643852645875496375698651748671976285
23142496323425351743453646285456475739656758684176
34221556924533266713564437782467554889357866599146
33859739644496184175551729528666283163977739427418
35135958944624616915573572712668468382377957949348
43587335415469844652657195576376821668748793277985
58262537826937364893714847591482595861259361697236
96856393331796741444281785255539289636664139174777
35323413594643452461575456357268656746837976785794
35722346434683345754579445686568155679767926678187
53476438526458754963756986517486719762859782187396
34253517434536462854564757396567586841767869795287
45332667135644377824675548893578665991468977611257
44961841755517295286662831639777394274188841538529
46246169155735727126684683823779579493488168151459
54698446526571955763768216687487932779859814388196
69373648937148475914825958612593616972361472718347
17967414442817852555392896366641391747775241285888
46434524615754563572686567468379767857948187896815
46833457545794456865681556797679266781878137789298
64587549637569865174867197628597821873961893298417
45364628545647573965675868417678697952878971816398
56443778246755488935786659914689776112579188722368
55172952866628316397773942741888415385299952649631
57357271266846838237795794934881681514599279262561
65719557637682166874879327798598143881961925499217
71484759148259586125936169723614727183472583829458
28178525553928963666413917477752412858886352396999
57545635726865674683797678579481878968159298917926
57944568656815567976792667818781377892989248891319
75698651748671976285978218739618932984172914319528
56475739656758684176786979528789718163989182927419
67554889357866599146897761125791887223681299833479";

    #[test]
    fn extend_test() {
        assert_eq!(parse(SAMPLE_EXTEND), extend(&parse(SAMPLE_INPUT)));
    }

    #[test]
    fn sample_test() {
        assert_eq!(315, run(SAMPLE_INPUT));
    }
}
