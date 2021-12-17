fn main() {
    println!("{}", min_fuel(&to_vector(include_str!("../input.txt"))));
}

/// Convert string of numbers separated by commas into a vector
fn to_vector(s: &str) -> Vec<i32> {
    s.split(',').map(|i| i.parse().unwrap()).collect()
}

/// Calculate median for this set of data
/// Mean accounts for outliers that have a heavier fuel cost - our answer will
/// be around here, but perhaps not the mean itself
fn mean(v: &[i32]) -> i32 {
    (v.iter().sum::<i32>() as f32 / v.len() as f32).round() as i32
}

/// Calculate the min amount of fuel required to align the submarines
fn min_fuel(v: &[i32]) -> i32 {
    let m = mean(v);

    // We will arbitrarily search 5% of the area surrounding the mean
    // This means we search 10 elements total
    let five_percent = (v.len() as f32 * 0.05).round() as i32;

    (m - five_percent..m + five_percent)
        .map(|i| v.iter().map(|j| (1..=(i - j).abs()).sum::<i32>()).sum())
        .min()
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    const WEB_INPUT: &str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn web_example() {
        assert_eq!(168, min_fuel(&to_vector(WEB_INPUT)));
    }
}
