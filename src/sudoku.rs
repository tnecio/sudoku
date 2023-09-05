use std::collections::BTreeSet;

#[derive(Clone)]
pub struct Sudoku([Option<u8>; 9 * 9]);

impl Sudoku {
    pub fn new(desc: &str) -> Self {
        //  format: "123___789\n" x 9
        let mut board: [Option<u8>; 9 * 9] = [None; 9 * 9];
        for (line, row_index) in desc.split("\n").zip(0..9) {
            for (char, col_index) in line.chars().zip(0..9) {
                board[col_index + row_index * 9] = match char.to_digit(10) {
                    None => None,
                    Some(x) => Some(x as u8),
                }
            }
        }
        Sudoku(board)
    }

    pub fn get(&self, x: usize, y: usize) -> Option<u8> {
        self.0[x + 9 * y]
    }

    pub fn set(&mut self, x: usize, y: usize, val: u8) -> () {
        self.0[x + 9 * y] = Some(val);
    }

    pub fn clear(&mut self, x: usize, y: usize) -> () {
        self.0[x + 9 * y] = None;
    }

    pub fn is_solved(&self) -> bool {
        for row_index in 0..9 {
            let different_digits: BTreeSet<_> = (0..9)
                .map(|col_index| self.get(col_index, row_index))
                .filter(Option::is_some)
                .collect();
            if different_digits.len() < 9 {
                return false;
            }
        }

        for col_index in 0..9 {
            let different_digits: BTreeSet<_> = (0..9)
                .map(|row_index| self.get(col_index, row_index))
                .filter(Option::is_some)
                .collect();
            if different_digits.len() < 9 {
                return false;
            }
        }

        for block_index in 0..9 {
            let bx = block_index / 3;
            let by = block_index % 3;
            let different_digits: BTreeSet<_> = (0..9)
                .map(|i| {
                    let x = i / 3;
                    let y = i % 3;
                    self.get(x + 3 * bx, y + 3 * by)
                })
                .filter(Option::is_some)
                .collect();
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
                None => String::from("_"),
            }));
            res.push_str("\n");
        }
        write!(f, "{}", res)
    }
}

#[test]
fn solved_sudoku_is_solved() {
    let sudoku = Sudoku::new(
        "123456789
456789123
789123456
234567891
567891234
891234567
345678912
678912345
912345678",
    );
    assert!(sudoku.is_solved());
}

#[test]
fn wrong_sudoku_is_not_solved() {
    let sudoku = Sudoku::new(
        "923456789
456789123
789123456
234567891
567891234
891234567
345678912
678912345
912345678",
    );
    assert!(!sudoku.is_solved());
}

#[test]
fn sudoku_with_missing_places_is_not_solved() {
    let sudoku = Sudoku::new(
        "_23456789
456789123
789123456
234567891
5678912_4
891234567
345678912
678912345
912345678",
    );
    assert!(!sudoku.is_solved());
}
