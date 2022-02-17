use std::collections::{BTreeSet};
use crate::SudokuBruteSolveResult::{Solved, NotYet};
use std::iter::FromIterator;

mod dll;
mod dlm;

struct Sudoku([Option<u8>; 9 * 9]);

impl Sudoku {
    fn new(desc: &str) -> Self {
    //  format: "123___789\n" x 9
        let mut board : [Option<u8>; 9 * 9] = [None; 9 * 9];
        for (line, row_index) in desc.split("\n").zip(0..9) {
            for (char, col_index) in line.chars().zip(0..9) {
                board[col_index + row_index * 9] = match char.to_digit(10) {
                    None => None,
                    Some(x) => Some(x as u8)
                }
            }
        }
        Sudoku(board)
    }

    fn get(&self, x: usize, y:usize) -> Option<u8> {
        self.0[x + 9 * y]
    }

    fn set(&mut self, x: usize, y: usize, val: u8) -> () {
        self.0[x + 9 * y] = Some(val);
    }

    fn clear(&mut self, x: usize, y: usize) -> () {
        self.0[x + 9 * y] = None;
    }

    fn is_solved(&self) -> bool {
        for row_index in 0..9 {
            let different_digits : BTreeSet<_> = (0..9).map(|col_index| self.get(col_index, row_index))
                .filter(Option::is_some).collect();
            if different_digits.len() < 9 {
                return false;
            }
        }

        for col_index in 0..9 {
            let different_digits : BTreeSet<_> = (0..9).map(|row_index| self.get(col_index, row_index))
                .filter(Option::is_some).collect();
            if different_digits.len() < 9 {
                return false;
            }
        }

        for block_index in 0..9 {
            let bx = block_index / 3;
            let by = block_index % 3;
            let different_digits : BTreeSet<_> = (0..9)
                .map(|i| {
                    let x = i / 3;
                    let y = i % 3;
                    self.get(x + 3 * bx, y + 3 * by)
                })
                .filter(Option::is_some).collect();
            if different_digits.len() < 9 {
                return false;
            }
        }

        true
    }
}

impl std::fmt::Display for Sudoku {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut res: String = String::new();
        for y in 0..9 {
            res.extend((0..9).map(|x| match self.get(x, y) {
                Some(digit) => digit.to_string(),
                None => String::from("_")
            }));
            res.push_str("\n");
        }
        write!(f, "{}", res)
    }
}

#[test]
fn solved_sudoku_is_solved() {
    let sudoku = Sudoku::new("123456789
456789123
789123456
234567891
567891234
891234567
345678912
678912345
912345678");
    assert!(sudoku.is_solved());
}

#[test]
fn wrong_sudoku_is_not_solved() {
    let sudoku = Sudoku::new("923456789
456789123
789123456
234567891
567891234
891234567
345678912
678912345
912345678");
    assert!(!sudoku.is_solved());
}

#[test]
fn sudoku_with_missing_places_is_not_solved() {
    let sudoku = Sudoku::new("_23456789
456789123
789123456
234567891
5678912_4
891234567
345678912
678912345
912345678");
    assert!(!sudoku.is_solved());
}

enum SudokuBruteSolveResult {
    Solved,
    NotYet
}

fn index_to_coords(index: usize) -> (usize, usize) {
    let x = index % 9;
    let y = index / 9;
    (x, y)
}

fn potential_digits(sudoku: &Sudoku, index: usize) -> BTreeSet<u8> {
    let (x, y) = index_to_coords(index);

    let mut res = BTreeSet::from_iter(1..=9);

    for i in 0..9 {
        // No same digits in the same row!
        if i != x {
            match sudoku.get(i, y) {
                None => (),
                Some(other) => { res.remove(&other); }
            }
        }
    }

    for i in 0..9 {
        // No same digits in the same column!
        if i != y {
            match sudoku.get(x, i) {
                None => (),
                Some(other) => { res.remove(&other); }
            }
        }
    }

    // No same digits in the same block!
    let bx = x / 3;
    let by = y / 3;
    let local_x = x % 3;
    let local_y = y % 3;
    for other_x in 0..3 {
        for other_y in 0..3 {
            if other_x != local_x && other_y != local_y {
                match sudoku.get(bx * 3 + other_x, by * 3 + other_y) {
                    None => (),
                    Some(other) => { res.remove(&other); }
                }
            }
        }
    }

    res
}

fn brute_solve_aux(sudoku: &mut Sudoku, index: usize) -> SudokuBruteSolveResult {
    if index >= 9 * 9 {
        if sudoku.is_solved() {
            return Solved;
        }
        return NotYet;
    }

    let (x, y) = index_to_coords(index);

    let current = sudoku.get(x, y);
    match current {
        Some(_digit) => brute_solve_aux(sudoku, index + 1),
        None => {
            for this_digit in potential_digits(sudoku, index) {
                sudoku.set(x, y, this_digit);
                match brute_solve_aux(sudoku, index + 1) {
                    Solved => { return Solved; }
                    _ => ()
                }
            }
            sudoku.clear(x, y);
            NotYet
        }
    }
}

fn brute_solve(sudoku: &mut Sudoku) -> () {
    brute_solve_aux(sudoku, 0);
}

#[test]
fn brute_solve_test() {
    let mut sudoku = Sudoku::new("__8627__9
___5_____
_3__9____
__69__3_2
______95_
1__8_____
____52_63
4___8____
___3__24_");
    brute_solve(&mut sudoku);
    assert!(sudoku.is_solved());
}



fn main() {
    let mut sudoku = Sudoku::new(
"__8627__9
___5_____
_3__9____
__69__3_2
______95_
1__8_____
____52_63
4___8____
___3__24_");
    brute_solve(&mut sudoku);
    println!("{}", sudoku);
}
