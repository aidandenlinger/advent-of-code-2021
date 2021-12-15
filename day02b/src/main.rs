use std::fs;

fn main() {
    // create the struct for our location
    let mut l = Location::new();

    let updates = parse_update_strs(&fs::read_to_string("input.txt").unwrap());

    l.update_vec(updates);

    println!("{}", l.mult())
}

#[derive(Debug)]
enum Update {
    Forward(i32),
    Up(i32),
    Down(i32),
}

/// Parse a string of updates into a vector of updates
fn parse_update_strs(i: &str) -> Vec<Update> {
    i.lines().map(str_to_update).collect()
}

/// Turn a single update string into an Update
fn str_to_update(s: &str) -> Update {
    match s.split_whitespace().collect::<Vec<_>>()[..] {
        ["forward", i] => Update::Forward(i.parse().unwrap()),
        ["down", i] => Update::Down(i.parse().unwrap()),
        ["up", i] => Update::Up(i.parse().unwrap()),
        _ => unreachable!(),
    }
}

/// Hold the current location
#[derive(Debug)]
struct Location {
    horiz: i32,
    vert: i32,
    aim: i32,
}

impl Location {
    /// Starting location of 0,0
    fn new() -> Location {
        Location {
            horiz: 0,
            vert: 0,
            aim: 0,
        }
    }

    /// Update the location by one Update
    fn update(&mut self, u: &Update) {
        match u {
            Update::Forward(i) => {
                self.horiz += i;
                self.vert += self.aim * i
            }
            Update::Down(i) => self.aim += i,
            Update::Up(i) => self.aim -= i,
        }
    }

    /// Update with a vector of updates
    fn update_vec(&mut self, updates: Vec<Update>) {
        for u in updates.iter() {
            self.update(u)
        }
    }

    /// Mult the two locational values together
    fn mult(&self) -> i32 {
        self.horiz * self.vert
    }
}

#[cfg(test)]
mod tests {
    /// The sample input from the website
    #[test]
    fn test_case() {
        use super::*;
        let updates = "\
forward 5
down 5
forward 8
up 3
down 8
forward 2";

        let mut l = Location::new();
        l.update_vec(parse_update_strs(updates));
        assert_eq!(900, l.mult());
    }
}
