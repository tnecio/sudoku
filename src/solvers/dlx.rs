use std::collections::HashMap;

use crate::{sudoku::*, dlx::DLMatrix};

fn row_index_to_cell_and_num(i: usize) -> (usize, usize, usize) {
    // Let's treat i as a number in 9-radix. Then the last digit is the number,
    // second digit is y, and first digit is x.
    let num = i % 9;
    let y = (i / 9) % 9;
    let x = (i / 9) / 9;
    (x, y, num)
}

pub fn dlx_solve(sudoku: &mut Sudoku) {
    // Build a DL matrix
    // 729 rows = 9 numbers x 81 cells
    // 324 columns = 81 + 81 + 81 + 81 (see below)
    let mut rows: Vec<Vec<bool>> = vec![];
    let mut map: HashMap<usize, (usize, usize, usize)> = HashMap::new(); // row index -> (x, y, num)
    for i in 0..729 {
        let mut row: Vec<bool> = vec![false; 324];
        // Each row represents putting a specific number into specific spot in the grid
        let (x, y, num) = row_index_to_cell_and_num(i);

        if let Some(existing_num) = sudoku.get(x, y) {
            if existing_num as usize != num + 1 {
                continue; // Don't include rows that contradict the hints
            }
        }

        // First 81 columns represent a condition that only one number is in a given spot
        row[x * 9 + y] = true;
        // Second 81 columns represent a condition that each number is in a given row (x) exactly once
        row[81 + x * 9 + num] = true;
        // Third 81 columns represent the same condition for rows
        row[81 * 2 + y * 9 + num] = true;
        // Last 81 columns represent the same condition for the blocks
        let block_index = (x / 3) * 3 + (y / 3);
        row[81 * 3 + block_index * 9 + num] = true;

        map.insert(rows.len(), (x, y, num));
        rows.push(row);
    }

    let mut dlm = DLMatrix::from_bool_rows(&rows);
    // dlm.print();
    let solutions = dlm.exact_cover();
    if solutions.len() != 1 {
        panic!("Expected 1 solution, found {}", solutions.len());
    }
    for i in solutions[0].iter() {
        let i = *i as usize;
        let (x, y, num) = map.get(&i).unwrap();
        sudoku.set(*x, *y, *num as u8 + 1);
    }
}
