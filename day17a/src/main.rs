use std::{cmp::max, ops::RangeInclusive};

const Y_MAX: i32 = 1000;

fn main() {
    println!("{}", run(include_str!("../input.txt")));
}

/// Given puzzle input, return the highest y position the probe can reach
/// when launched while still landing within the target range.
fn run(s: &str) -> i32 {
    let r = Ranges::new(s);

    // Find the first v_x that results in a final x coord within the x range
    // Final x coord: 1 + 2 + ... + v_x
    let x_min = inv_arith_sum(*r.x.start());

    // the x_max: the final x coordinate of the target + 1. This would mean
    // on the first shot, we overshoot the entire target
    let x_max = r.x.end() + 1;

    // y_min: the first point that shoots underneath the target, missing it entirely
    // This only works because we know the y target starting variable is negative,
    // so it can't go back up! Would need y_max like math otherwise
    let y_min = r.y.start() - 1;

    // y_max = tried to be mathematical about this, couldn't find a good way to
    // KNOW that at some point, every velocity after will fail. yoloing on 1000
    // i limited x, that's pretty good, but it is very annoying to know that
    // this isn't PROOF that there isn't a better velocity somewhere out there
    // I have math blogs to read later
    *(x_min..x_max)
        .map(|x| (y_min..Y_MAX).map(|y| r.launch((x, y))).collect::<Vec<_>>())
        .collect::<Vec<_>>()
        .iter()
        .flatten()
        .flatten()
        .max()
        .unwrap()
}

/// Solve for n such that n is the first int where 1 + 2 + ... + n > x.
fn inv_arith_sum(x: i32) -> i32 {
    // We can solve for 1 + 2 + ... + n as n(n+1)/2 since it is an arithmetic sum.
    // Our goal is to solve for n(n-1)/2 > x. We can expand this to
    // n^2 + n - 2x > 0. Quadratic equation gives us
    // -1 +/- sqrt(1 + 8x) / 2 > 0.
    // We can ignore the - from +/-, since -1 - positive num is negative.
    // Thus, we solve the equation here, and ceiling it so it remains > 0.
    ((-1_f32 + ((1 + (8 * x)) as f32).sqrt()) * 0.5).ceil() as i32
}

#[derive(Debug, PartialEq)]
/// Ranges for the target area.
struct Ranges {
    x: RangeInclusive<i32>,
    y: RangeInclusive<i32>,
}

impl Ranges {
    /// Given puzzle input, parse out into the Ranges we need.
    fn new(s: &str) -> Ranges {
        match s
            .split_whitespace()
            .skip(2)
            .map(|s| {
                match s
                    .split_once('=')
                    .unwrap()
                    .1
                    .split("..")
                    .map(|i| {
                        i.chars()
                            // There's definitely a better way to do this, but
                            // clear out the , at the end of x_max
                            .filter(|&c| c != ',')
                            .collect::<String>()
                            .parse()
                            .unwrap()
                    })
                    .collect::<Vec<_>>()[..]
                {
                    [x, y] => (x, y),
                    _ => unreachable!(),
                }
            })
            .collect::<Vec<_>>()[..]
        {
            [(x1, x2), (y1, y2)] => Ranges {
                x: (x1..=x2),
                y: (y1..=y2),
            },
            _ => unreachable!(),
        }
    }

    /// Given a launch velocity, return the highest Y dimension if the prove
    /// reached the target area, or None if it does not ever reach the target.
    fn launch(&self, (v_x, v_y): (i32, i32)) -> Option<i32> {
        let (mut x, mut y) = (0, 0);
        let (mut v_x, mut v_y) = (v_x, v_y);
        let mut max_y = 0;

        // while we haven't passed the target (not too far right or down)
        while x <= *self.x.end() && y >= *self.y.start() {
            // update cycle
            x += v_x;
            y += v_y;
            v_x = if v_x > 0 { v_x - 1 } else { 0 };
            v_y -= 1;

            // update max_y
            max_y = max(y, max_y);

            // Have we landed in the target?
            if self.x.contains(&x) && self.y.contains(&y) {
                return Some(max_y);
            }
        }

        // We passed the target without landing, return None
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "target area: x=20..30, y=-10..-5";

    #[test]
    fn step_test() {
        let r = Ranges::new(SAMPLE_INPUT);

        assert_eq!(Some(3), r.launch((7, 2)));
        assert_eq!(Some(6), r.launch((6, 3)));
        assert_eq!(Some(0), r.launch((9, 0)));
        assert_eq!(None, r.launch((17, -4)));
    }

    #[test]
    fn parse_test() {
        let ans = Ranges {
            x: (20..=30),
            y: (-10..=-5),
        };
        assert_eq!(ans, Ranges::new(SAMPLE_INPUT));
    }

    #[test]
    fn sample_test() {
        assert_eq!(45, run(SAMPLE_INPUT));
    }
}
