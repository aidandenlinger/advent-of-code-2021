const BIRTH: i32 = 0;
const POST_BIRTH: i32 = 6;
const NEW_FISH: i32 = 8;
const DAYS: i32 = 80;

fn main() {
    let mut fishes = to_vec(include_str!("../input.txt"));

    for _ in 0..DAYS {
        fishes = day_update(fishes);
    }

    println!("{}", fishes.len());
}

/// Convert string of comma separated numbers into Vec
fn to_vec(s: &str) -> Vec<i32> {
    s.split(',').map(|i| i.parse().unwrap()).collect()
}

/// Update the list from one day.
fn day_update(mut fishes: Vec<i32>) -> Vec<i32> {
    let mut new_fish_count = 0;

    // Update current fish
    for fish in &mut fishes {
        if *fish == BIRTH {
            *fish = POST_BIRTH;
            new_fish_count += 1;
        } else {
            *fish -= 1;
        }
    }

    // Add new fish
    fishes.append(&mut vec![NEW_FISH; new_fish_count]);

    fishes
}

#[cfg(test)]
mod test {
    use super::*;

    const WEB_EXAMPLE: &str = "3,4,3,1,2";

    #[test]
    fn web_example() {
        let mut fishes = to_vec(WEB_EXAMPLE);

        for _ in 0..80 {
            fishes = day_update(fishes);
        }

        assert_eq!(5934, fishes.len());
    }
}
