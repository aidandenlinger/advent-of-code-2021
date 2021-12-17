const NUMBER_OF_STAGES: usize = 9;
const BIRTH: usize = 0;
const POST_BIRTH: usize = 6;
const NEW_FISH: usize = 8;
const NUMBER_OF_DAYS: usize = 256;

fn main() {
    let mut fishes = to_array(include_str!("../input.txt"));

    for _ in 0..NUMBER_OF_DAYS {
        day_update(&mut fishes);
    }

    println!("{}", fishes.iter().sum::<u64>());
}

/// Convert string of comma separated numbers into array
/// Index represents the stage, value is the number of fishes at that stage
fn to_array(s: &str) -> [u64; NUMBER_OF_STAGES] {
    let mut arr = [0; NUMBER_OF_STAGES];

    let nums: Vec<u64> = s.split(',').map(|i| i.parse().unwrap()).collect();

    for (i, n) in arr.iter_mut().enumerate() {
        *n = nums.iter().filter(|&j| j == &(i as u64)).count() as u64
    }

    arr
}

/// Create array of next state of fishes
fn day_update(f: &mut [u64; 9]) {
    let birthing = f[BIRTH];

    // Move fish from timer i+1 to timer i
    for i in 0..NUMBER_OF_STAGES - 1 {
        f[i] = f[i + 1]
    }

    f[POST_BIRTH] += birthing; // fish who just birthed go to POST_BIRTH
    f[NEW_FISH] = birthing; // each fish who birthed add 1 new fish - overwrite it
}

#[cfg(test)]
mod test {
    use super::*;

    const WEB_EXAMPLE: &str = "3,4,3,1,2";

    #[test]
    fn web_example() {
        let mut fishes = to_array(WEB_EXAMPLE);

        for _ in 0..NUMBER_OF_DAYS {
            day_update(&mut fishes);
        }

        assert_eq!(26984457539, fishes.iter().sum::<u64>());
    }
}
