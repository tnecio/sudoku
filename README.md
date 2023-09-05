Small collection of Sudoku solvers in Rust.

Run with `--brute` to use the brute-force with [backtracking](https://en.wikipedia.org/wiki/Backtracking) algorithm.

Use `--dlx` to use the Donald Knuth's [X algorithm](https://en.wikipedia.org/wiki/Knuth%27s_Algorithm_X) with dancing links (DLX)

Example of usage: `cat data/sudoku1.txt | cargo run -- --dlx`

Test with `cargo test`.