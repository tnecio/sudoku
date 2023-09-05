use std::{collections::HashSet, env, io};
use sudoku::Sudoku;

use solvers::brute::brute_solve;
use solvers::dlx::dlx_solve;

mod dlx;
mod solvers;
pub(crate) mod sudoku;

fn main() -> io::Result<()> {
    let args: HashSet<String> = env::args().collect();
    let mut input_str = String::new();
    let stdin = io::stdin();
    for _ in 0..9 {
        stdin.read_line(&mut input_str)?;
    }
    let mut sudoku = Sudoku::new(&input_str);
    if args.contains("--brute") {
        brute_solve(&mut sudoku);
        println!("{}", sudoku);
    }
    if args.contains("--dlx") {
        dlx_solve(&mut sudoku);
        println!("{}", sudoku);
    }
    Ok(())
}
