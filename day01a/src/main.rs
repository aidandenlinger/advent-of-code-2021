use std::{error::Error, fs};

fn main() {
    println!("{}", count_increase(&file_to_vec("input.txt").unwrap()));
}

/// Turn file of numbers into vector of numbers
fn file_to_vec(f: &str) -> Result<Vec<i32>, Box<dyn Error>> {
    Ok(fs::read_to_string(f)?
        .lines()
        .map(|i| i.parse().unwrap())
        .collect::<Vec<i32>>())
}

fn count_increase(v: &Vec<i32>) -> i32 {
    v.windows(2)
        .fold(0, |acc, w| if w[1] > w[0] { acc + 1 } else { acc })
}
