use std::{error::Error, fs};

fn main() {
    println!(
        "{}",
        //count_increase(&vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263])
        count_increase(&file_to_vec("input.txt").unwrap())
    )
}

/// Turn file of numbers into vector of numbers
fn file_to_vec(f: &str) -> Result<Vec<i32>, Box<dyn Error>> {
    Ok(fs::read_to_string(f)?
        .lines()
        .map(|i| i.parse().unwrap())
        .collect::<Vec<i32>>())
}

fn count_increase(v: &Vec<i32>) -> i32 {
    // we want to compare (a[0] + a[1] + a[2]) vs (a[1] + a[2] + a[3])
    // this can be totally simplified down: it shares a[1] and a[2]
    // so really, we just want to compare a[0] vs a[3]
    // get windows of 4 to do just that
    v.windows(4).filter(|w| w[0] < w[3]).count() as i32
}
