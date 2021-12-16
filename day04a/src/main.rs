use std::{collections::HashMap, fs};

fn main() {
    let s = fs::read_to_string("input.txt").unwrap();

    let (nums, boards) = parse_input(&s);

    let (last_called, board) = find_fast_board(&nums, boards);

    println!("{}", last_called * sum_unmarked(&board))
}

/// Parse an input string into the numbers that will be marked and all boards
fn parse_input(s: &str) -> (Vec<i32>, Vec<Board>) {
    let mut iter = s.split("\n\n");

    // First input is the drawn numbers
    let drawn_nums = iter
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    let boards = iter.map(Board::new).collect();

    (drawn_nums, boards)
}

/// Sum up the nonmarked numbers (this is bad and I'm sorry)
fn sum_unmarked(b: &Board) -> i32 {
    b.index
        .iter()
        .fold(0, |acc, (&n, &i)| if !b.board[i] { acc + n } else { acc })
}

/// Find the board that is completed the fastest.
fn find_fast_board(nums: &[i32], mut boards: Vec<Board>) -> (i32, Board) {
    for &i in nums {
        for b in &mut boards {
            b.mark(i);
        }

        if let Some(b) = boards.iter().find(|b| b.win()) {
            return (i, b.clone());
        }
    }

    unreachable!()
}

#[derive(Debug, Clone)]
/// Bingo board
struct Board {
    index: HashMap<i32, usize>,
    board: [bool; 25],
}

impl Board {
    /// Given a string of a board, creates a new board.
    fn new(s: &str) -> Board {
        let nums = s
            .lines()
            .map(|s| {
                s.split_whitespace()
                    .map(|i| i.parse().unwrap())
                    .collect::<Vec<_>>()
            })
            .flatten();

        let mut index_map = HashMap::new();

        for (i, num) in nums.enumerate() {
            index_map.insert(num, i);
        }

        Board {
            index: index_map,
            board: [false; 25],
        }
    }

    /// If the board contains the number, marks it as true.
    /// Silently does nothing if the board does not contain the number.
    fn mark(&mut self, n: i32) {
        if let Some(i) = self.index.get(&n) {
            self.board[*i] = true;
        }
    }

    /// Check if this board has any winning conditions
    fn win(&self) -> bool {
        // Horizontal win case: are any of the rows all true?
        self.board
            .chunks_exact(5)
            .into_iter()
            .map(|row| row.iter().all(|&x| x))
            .any(|x| x)
        ||
        // Vertical win case: are any of the columns all true?
        (0..5)
            .map(|i| self.board.iter().skip(i).step_by(5).all(|&x| x))
            .any(|x| x)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    const SIMPLE_BOARD: &str =
        "1 2 3 4 5\n6 7 8 9 10\n11 12 13 14 15\n16 17 18 19 20\n21 22 23 24 25";

    const WEB_BOARD: &str = "\
7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

    #[test]
    fn web_test() {
        let (nums, boards) = parse_input(WEB_BOARD);

        let (last_called, board) = find_fast_board(&nums, boards);

        assert_eq!(4512, last_called * sum_unmarked(&board));
    }

    #[test]
    /// Create a test board and ensure Board holds what's expected
    fn create_board() {
        let b = Board::new(SIMPLE_BOARD);

        assert_eq!([false; 25], b.board);
        for i in 1..26 {
            assert_eq!(i - 1, b.index[&(i as i32)]);
        }
    }

    #[test]
    /// Ensure correct number is marked
    fn mark() {
        let mut b = Board::new(SIMPLE_BOARD);

        b.mark(5);

        let mut ans = [false; 25];
        ans[4] = true;

        assert_eq!(ans, b.board);
    }

    #[test]
    fn win_hori() {
        let mut b = Board::new(SIMPLE_BOARD);

        assert!(!b.win());

        for i in [3, 8, 13, 18] {
            b.mark(i);
            assert!(!b.win());
        }

        b.mark(23);
        assert!(b.win())
    }

    #[test]
    fn win_vert() {
        let mut b = Board::new(SIMPLE_BOARD);

        assert!(!b.win());

        for i in [6, 7, 8, 9] {
            b.mark(i);
            assert!(!b.win());
        }

        b.mark(10);
        assert!(b.win())
    }
}
