fn main() {
    println!("{}", min_fuel(&mut to_vector(include_str!("../input.txt"))));
}

/// Convert string of numbers separated by commas into a vector
fn to_vector(s: &str) -> Vec<i32> {
    s.split(',').map(|i| i.parse().unwrap()).collect()
}

/// Find a median of a vector of numbers
/// A median is guaranteed to optimize, for x, sum(|arr_elements-x|)
/// https://math.stackexchange.com/questions/113270/the-median-minimizes-the-sum-of-absolute-deviations-the-ell-1-norm
fn median(v: &mut Vec<i32>) -> i32 {
    v.sort_unstable();
    v[v.len() / 2]
}

/// Calculate the min amount of fuel required to align the submarines
fn min_fuel(v: &mut Vec<i32>) -> i32 {
    let m = median(v);
    v.iter().map(|i| (i - m).abs()).sum()
}

#[cfg(test)]
mod test {
    use super::*;

    const WEB_INPUT: &str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn web_example() {
        assert_eq!(37, min_fuel(&mut to_vector(WEB_INPUT)));
    }
}
