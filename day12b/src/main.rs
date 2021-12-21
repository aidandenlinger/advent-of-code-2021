use std::collections::HashMap;

const START: &str = "start";
const END: &str = "end";

fn main() {
    println!("{}", run(include_str!("../input.txt")));
}

/// Given puzzle cave input, determine the number of paths that can visit one
/// small cave twice, and all other small caves once at most.
fn run(s: &str) -> u32 {
    let g = parse(s);

    all_paths(g)
}

#[derive(Clone)]
struct Path<'a> {
    path: Vec<&'a str>,
    visited_small_twice: bool,
}

impl Path<'_> {
    // Generate a new path that starts at START
    fn new() -> Path<'static> {
        Path {
            path: vec![START],
            visited_small_twice: false,
        }
    }
}

/// Given a graph, return the number of paths that visit small caves at most
/// once, starting from START and ending at END.
fn all_paths(g: HashMap<&str, Node>) -> u32 {
    // Key idea: do BFS but store the entire path rather than just the node in the queue
    // Start by enqueuing our start node
    let mut queue = vec![Path::new()];
    let mut paths = vec![];

    while !queue.is_empty() {
        let curr_path = queue.pop().unwrap();
        let curr = curr_path.path.last().unwrap();

        if curr == &END {
            // This is a complete path! Don't add anything to queue
            paths.push(curr_path);
            continue;
        }

        for n in g[*curr].neighbors.iter().map(|&s| &g[s]) {
            if !curr_path.visited_small_twice
                && n.small
                && n.name != START
                && curr_path.path.iter().filter(|&s| s == &n.name).count() == 1
            {
                // Special case: we have a small neighbor that we've visited once
                // before. Make new path with it and note that we've visited
                // before
                queue.push({
                    let mut next_path = curr_path.clone();
                    next_path.path.push(n.name);
                    next_path.visited_small_twice = true;
                    next_path
                });
            } else if !n.small || !curr_path.path.contains(&n.name) {
                // Otherwise, same logic as part 1 - not small nodes can be
                // visited as many times as we want, or we can add small nodes
                // we've never visited before
                queue.push({
                    let mut next_path = curr_path.clone();
                    next_path.path.push(n.name);
                    next_path
                });
            }
        }
    }

    paths.len().try_into().unwrap()
}

/// Given puzzle cave input, parse into a hashmap of string to node.
fn parse(s: &str) -> HashMap<&str, Node> {
    let mut g = HashMap::new();

    for l in s.lines() {
        let (i1, i2) = l.split_once('-').unwrap();

        let n1 = g.entry(i1).or_insert_with(|| Node::new(i1));
        n1.neighbors.push(i2);

        let n2 = g.entry(i2).or_insert_with(|| Node::new(i2));
        n2.neighbors.push(i1);
    }

    g
}

/// Node in a graph. Is responsible for the strings within itself.
struct Node<'a> {
    neighbors: Vec<&'a str>,
    name: &'a str,
    small: bool,
}

impl Node<'_> {
    // Create a blank node with its given name
    fn new(s: &str) -> Node {
        Node {
            neighbors: Vec::new(),
            name: s,
            small: s.chars().next().unwrap().is_lowercase(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const WEB_INPUT_1: &str = "\
start-A
start-b
A-c
A-b
b-d
A-end
b-end";

    const WEB_INPUT_2: &str = "\
dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc";

    const WEB_INPUT_3: &str = "\
fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW";

    #[test]
    fn web_example_1() {
        assert_eq!(36, run(WEB_INPUT_1));
    }

    #[test]
    fn web_example_2() {
        assert_eq!(103, run(WEB_INPUT_2));
    }

    #[test]
    fn web_example_3() {
        assert_eq!(3509, run(WEB_INPUT_3));
    }
}
