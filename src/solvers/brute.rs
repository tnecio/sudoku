use crate::sudoku::*;
use std::collections::BTreeSet;
use std::iter::FromIterator;

enum SudokuBruteSolveResult {
    Solved,
    NotYet,
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
                Some(other) => {
                    res.remove(&other);
                }
            }
        }
    }

    for i in 0..9 {
        // No same digits in the same column!
        if i != y {
            match sudoku.get(x, i) {
                None => (),
                Some(other) => {
                    res.remove(&other);
                }
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
                    Some(other) => {
                        res.remove(&other);
                    }
                }
            }
        }
    }

    res
}

fn brute_solve_aux(sudoku: &mut Sudoku, index: usize) -> SudokuBruteSolveResult {
    if index >= 9 * 9 {
        if sudoku.is_solved() {
            return SudokuBruteSolveResult::Solved;
        }
        return SudokuBruteSolveResult::NotYet;
    }

    let (x, y) = index_to_coords(index);

    let current = sudoku.get(x, y);
    match current {
        Some(_digit) => brute_solve_aux(sudoku, index + 1),
        None => {
            for this_digit in potential_digits(sudoku, index) {
                sudoku.set(x, y, this_digit);
                match brute_solve_aux(sudoku, index + 1) {
                    SudokuBruteSolveResult::Solved => {
                        return SudokuBruteSolveResult::Solved;
                    }
                    _ => (),
                }
            }
            sudoku.clear(x, y);
            SudokuBruteSolveResult::NotYet
        }
    }
}

pub fn brute_solve(sudoku: &mut Sudoku) {
    brute_solve_aux(sudoku, 0);
}

#[test]
fn brute_solve_test() {
    let mut sudoku = Sudoku::new(
        "__8627__9
___5_____
_3__9____
__69__3_2
______95_
1__8_____
____52_63
4___8____
___3__24_",
    );
    brute_solve(&mut sudoku);
    assert!(sudoku.is_solved());
}
