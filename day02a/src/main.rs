use std::{error::Error, fs};

fn main() {
    println!(
        "{}",
        calculate_loc(&fs::read_to_string("input.txt").unwrap()).mult(),
    )
}

/// The sample input from the website
fn _test() {
    let s = "\
forward 5
down 5
forward 8
up 3
down 8
forward 2";
    println!("{}", calculate_loc(s).mult());
}

#[derive(Debug)]
enum Update {
    Forward(i32),
    Up(i32),
    Down(i32),
}

/// Calculate the location based off a vector of updates
fn calculate_loc(u: &str) -> Location {
    let mut s = Location::new();
    s.update_vec(parse_update_strs(u));
    s
}

/// Parse a string of updates into a vector of updates
fn parse_update_strs(i: &str) -> Vec<Update> {
    i.lines().map(|i| str_to_update(i).unwrap()).collect()
}

/// Turn a single update string into an Update
fn str_to_update(i: &str) -> Result<Update, Box<dyn Error>> {
    match i.split_whitespace().collect::<Vec<_>>()[..] {
        ["forward", i] => Ok(Update::Forward(i.parse().unwrap())),
        ["down", i] => Ok(Update::Down(i.parse().unwrap())),
        ["up", i] => Ok(Update::Up(i.parse().unwrap())),
        _ => Err("incorrect instruction format".into()),
    }
}

/// Hold the current location
#[derive(Debug)]
struct Location {
    horiz: i32,
    vert: i32,
}

impl Location {
    /// Starting location of 0,0
    fn new() -> Location {
        Location { horiz: 0, vert: 0 }
    }

    /// Update the location by one Update
    fn update(&mut self, u: &Update) {
        match u {
            Update::Forward(i) => self.horiz += i,
            Update::Down(i) => self.vert += i,
            Update::Up(i) => self.vert -= i,
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
