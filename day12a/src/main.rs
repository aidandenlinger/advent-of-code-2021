use std::collections::HashMap;

const START: &str = "start";
const END: &str = "end";

fn main() {
    println!("{}", run(include_str!("../input.txt")));
}

/// Given puzzle cave input, determine the number of paths that visit small
/// caves at most once.
fn run(s: &str) -> u32 {
    let g = parse(s);

    all_paths(g)
}

/// Given a graph, return the number of paths that visit small caves at most
/// once, starting from START and ending at END.
fn all_paths(g: HashMap<&str, Node>) -> u32 {
    // Key idea: do BFS but store the entire path rather than just the node in the queue
    // Start by enqueuing our start node
    let mut queue = vec![vec![START]];
    let mut paths = vec![];

    while !queue.is_empty() {
        let curr_path = queue.pop().unwrap();
        let curr = curr_path.last().unwrap();

        if curr == &END {
            // This is a complete path! Don't add anything to queue
            paths.push(curr_path);
            continue;
        }

        for n in g[*curr].neighbors.iter().map(|&s| &g[s]) {
            if !n.small || !curr_path.contains(&n.name) {
                queue.push({
                    let mut next_path = curr_path.clone();
                    next_path.push(n.name);
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
    fn basic_parse_test() {
        let g = parse(WEB_INPUT_1);

        // this loop spot checks "A" - not a through test, but I
        // checked everything with a dbg! earlier
        for n in ["start", "c", "b", "end"] {
            assert!(g[&"A"].neighbors.contains(&n));
        }

        assert!(g[&"A"].neighbors.len() == 4);
    }

    #[test]
    fn web_example_1() {
        assert_eq!(10, run(WEB_INPUT_1));
    }

    #[test]
    fn web_example_2() {
        assert_eq!(19, run(WEB_INPUT_2));
    }

    #[test]
    fn web_example_3() {
        assert_eq!(226, run(WEB_INPUT_3));
    }
}
