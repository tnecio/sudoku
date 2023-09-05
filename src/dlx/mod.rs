use std::collections::HashMap;

use prettytable::{Cell, Row, Table};

enum Dir {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug)]
pub struct DLMatrix {
    right: Vec<u16>,
    left: Vec<u16>,
    up: Vec<u16>,
    down: Vec<u16>,
    column: Vec<u16>,           // used as x if the node is column header
    y: Vec<i16>, // used as size if the node is column header, i.e. when the value is < 0
    columns: HashMap<u16, u16>, // column node for given x
    rows: HashMap<u16, u16>, // first cell for given y
}

impl DLMatrix {
    pub fn new() -> Self {
        DLMatrix {
            columns: HashMap::new(),
            rows: HashMap::new(),
            right: vec![0],
            left: vec![0],
            up: vec![0],
            down: vec![0],
            column: vec![0],
            y: vec![0],
        }
    }

    fn set(&mut self, src: u16, direction: Dir, dst: u16) {
        match direction {
            Dir::Up => self.up[src as usize] = dst,
            Dir::Right => self.right[src as usize] = dst,
            Dir::Down => self.down[src as usize] = dst,
            Dir::Left => self.left[src as usize] = dst,
        }
    }

    #[inline]
    fn get_neigh_ptr(&self, ptr: u16, direction: Dir) -> u16 {
        match direction {
            Dir::Up => self.up[ptr as usize],
            Dir::Right => self.right[ptr as usize],
            Dir::Down => self.down[ptr as usize],
            Dir::Left => self.left[ptr as usize],
        }
    }

    #[inline]
    fn get_column_ptr(&self, ptr: u16) -> u16 {
        let y = self.y[ptr as usize];
        if y < 0 {
            ptr
        } else {
            self.column[ptr as usize]
        }
    }

    #[inline(always)]
    fn root_ptr(&self) -> u16 {
        0
    }

    // new_node_factory(ptr) must return a DLNode struct that has valid pointers
    fn add_node<F>(&mut self, f: F) -> u16
    where
        F: Fn(u16) -> (u16, u16, u16, u16, u16, i16),
    {
        let ptr = self.y.len() as u16;
        let (left, up, right, down, column, y) = f(ptr);
        self.left.push(left);
        self.up.push(up);
        self.right.push(right);
        self.down.push(down);
        self.y.push(y);
        self.column.push(column);

        self.set(self.get_neigh_ptr(ptr, Dir::Left), Dir::Right, ptr);
        self.set(self.get_neigh_ptr(ptr, Dir::Up), Dir::Down, ptr);
        self.set(self.get_neigh_ptr(ptr, Dir::Right), Dir::Left, ptr);
        self.set(self.get_neigh_ptr(ptr, Dir::Down), Dir::Up, ptr);

        ptr
    }

    fn add_column(&mut self, x: u16) -> u16 {
        if self.columns.contains_key(&x) {
            return *self.columns.get(&x).unwrap();
        }
        let (root_left_ptr, root_ptr) = (
            self.get_neigh_ptr(self.root_ptr(), Dir::Left),
            self.root_ptr(),
        );
        let ptr = self.add_node(|ptr| (root_left_ptr, ptr, root_ptr, ptr, x, -1));
        self.columns.insert(x, ptr);
        ptr
    }

    fn add_cell(&mut self, x: u16, y: u16) -> u16 {
        let col_ptr = if !self.columns.contains_key(&x) {
            self.add_column(x)
        } else {
            *self.columns.get(&x).unwrap()
        };
        self.y[col_ptr as usize] -= 1; // Increase size by one; TODO: separate to a different function
        let col_up_ptr = self.get_neigh_ptr(col_ptr, Dir::Up);

        let row_ptrs = if self.rows.contains_key(&y) {
            let row_start_ptr = *self.rows.get(&y).unwrap();
            let row_end_ptr = self.get_neigh_ptr(row_start_ptr, Dir::Left);
            (Some(row_start_ptr), Some(row_end_ptr))
        } else {
            (None, None)
        };

        let ptr = self.add_node(|ptr| {
            (
                row_ptrs.1.unwrap_or(ptr),
                col_up_ptr,
                row_ptrs.0.unwrap_or(ptr),
                col_ptr,
                col_ptr,
                y as i16,
            )
        });

        if row_ptrs == (None, None) {
            self.rows.insert(y, ptr);
        }

        ptr
    }

    pub fn from_bool_rows(rows: &Vec<Vec<bool>>) -> Self {
        let mut res = Self::new();
        for (y, row) in rows.iter().enumerate() {
            for (x, value) in row.iter().enumerate() {
                if *value {
                    res.add_cell(x as u16, y as u16);
                }
            }
        }
        res
    }

    fn unlink_left_right(&mut self, ptr: u16) {
        let left = self.get_neigh_ptr(ptr, Dir::Left);
        let right = self.get_neigh_ptr(ptr, Dir::Right);
        self.set(right, Dir::Left, left);
        self.set(left, Dir::Right, right);
    }

    fn relink_left_right(&mut self, ptr: u16) {
        let left = self.get_neigh_ptr(ptr, Dir::Left);
        let right = self.get_neigh_ptr(ptr, Dir::Right);
        self.set(right, Dir::Left, ptr);
        self.set(left, Dir::Right, ptr);
    }

    fn unlink_up_down(&mut self, ptr: u16) {
        let up = self.get_neigh_ptr(ptr, Dir::Up);
        let down = self.get_neigh_ptr(ptr, Dir::Down);
        self.set(down, Dir::Up, up);
        self.set(up, Dir::Down, down);
        let col = self.get_column_ptr(ptr);
        if col != ptr {
            self.y[ptr as usize] += 1; // Decrease size by one. Todo: separate function.
        }
    }

    fn relink_up_down(&mut self, ptr: u16) {
        let up = self.get_neigh_ptr(ptr, Dir::Up);
        let down = self.get_neigh_ptr(ptr, Dir::Down);
        self.set(down, Dir::Up, ptr);
        self.set(up, Dir::Down, ptr);
        let col = self.get_column_ptr(ptr);
        if col != ptr {
            self.y[ptr as usize] -= 1; // Increase size by one. Todo: separate function.
        }
    }

    // Solution = set of columns' x coordinates
    pub fn exact_cover(&mut self) -> Vec<Vec<u16>> {
        let mut o_vals: Vec<u16> = Vec::new();
        let mut solutions: Vec<Vec<u16>> = Vec::new();
        self.exact_cover_rec(0, &mut o_vals, &mut solutions);
        solutions
    }

    fn exact_cover_rec(
        &mut self,
        k: u16,
        partial_solution: &mut Vec<u16>,
        solutions: &mut Vec<Vec<u16>>,
    ) {
        // If the matrix A has no columns, the current partial solution is a valid solution; terminate successfully.
        if self.get_neigh_ptr(self.root_ptr(), Dir::Right) == self.root_ptr() {
            solutions.push(self.current_solution(partial_solution));
            return;
        }

        let c: u16 = self.choose_column();

        // Try every row r that itersects the column c: (this can be parallelized if we clone the matrix)
        let mut r = c;
        loop {
            r = self.get_neigh_ptr(r, Dir::Down);

            if r == c {
                break;
            }

            // Include row r in the partial solution.
            partial_solution.push(r);

            // Every column that is handled by row r is no longer in the equation.
            // Remove all such columns AND all rows that also intersect such columns.
            // We say: cover all such columns.
            let mut j = r;
            loop {
                self.cover(self.get_column_ptr(j));
                j = self.get_neigh_ptr(j, Dir::Right);
                if j == r {
                    break;
                }
            }

            self.exact_cover_rec(k + 1, partial_solution, solutions);

            // Undo covering the columns
            loop {
                j = self.get_neigh_ptr(j, Dir::Left);
                self.uncover(self.get_column_ptr(j));
                if j == r {
                    break;
                }
            }

            partial_solution.pop();
        }
    }

    fn choose_column(&self) -> u16 {
        let mut s = i16::MAX;
        let mut j = self.root_ptr();
        let mut c = j;
        loop {
            j = self.get_neigh_ptr(j, Dir::Right);
            if j == self.root_ptr() {
                break;
            }
            let size = -self.y[j as usize] - 1;
            if size < s {
                s = size;
                c = j;
            }
        }
        c
    }

    // Cover the column: delete it and all rows that intersect it.
    fn cover(&mut self, col_ptr: u16) {
        self.unlink_left_right(col_ptr);
        let mut row_ptr = self.get_neigh_ptr(col_ptr, Dir::Down);
        while row_ptr != col_ptr {
            let mut j = self.get_neigh_ptr(row_ptr, Dir::Right);
            while j != row_ptr {
                self.unlink_up_down(j);
                j = self.get_neigh_ptr(j, Dir::Right);
            }
            row_ptr = self.get_neigh_ptr(row_ptr, Dir::Down);
        }
    }

    // Uncover the column: undelete it and all rows that intersect it.
    fn uncover(&mut self, col_ptr: u16) {
        let mut row_ptr = self.get_neigh_ptr(col_ptr, Dir::Up);
        while row_ptr != col_ptr {
            let mut j = self.get_neigh_ptr(row_ptr, Dir::Left);
            while j != row_ptr {
                self.relink_up_down(j);
                j = self.get_neigh_ptr(j, Dir::Left);
            }
            row_ptr = self.get_neigh_ptr(row_ptr, Dir::Up);
        }

        self.relink_left_right(col_ptr);
    }

    fn current_solution(&mut self, partial_solution: &mut Vec<u16>) -> Vec<u16> {
        let mut res: Vec<u16> = Vec::new();
        for &ptr in partial_solution.iter() {
            res.push(self.y[ptr as usize] as u16);
        }
        res
    }

    #[allow(dead_code)] // useful for debugging
    pub fn print(&self) {
        let root_ptr = self.root_ptr();
        let mut columns = HashMap::new();
        let mut cells = HashMap::new();
        let mut col_ptr = root_ptr;
        let mut max_x = 0;
        let mut max_y = 0;
        loop {
            col_ptr = self.get_neigh_ptr(col_ptr, Dir::Right);
            if col_ptr == root_ptr {
                break;
            }
            let x = self.column[col_ptr as usize];
            columns.insert(x, col_ptr);
            if x > max_x {
                max_x = x;
            }

            let mut ptr = col_ptr;
            loop {
                ptr = self.get_neigh_ptr(ptr, Dir::Down);
                if ptr == col_ptr {
                    break;
                }
                let y = self.y[ptr as usize];
                cells.insert((x, y), ptr);
                if y > max_y {
                    max_y = y;
                }
            }
        }

        let mut lines = Vec::new();
        let mut column_names = Vec::new();
        column_names.push(String::from(""));
        for column_no in 0..=max_x {
            column_names.push(format!("{}", column_no));
        }
        lines.push(column_names);

        let mut column_ptrs = vec![String::from("C")];
        for column_no in 0..=max_x {
            column_ptrs.push(if let Some(ptr) = columns.get(&column_no) {
                format!("{}", ptr)
            } else {
                String::from("")
            });
        }
        lines.push(column_ptrs);

        for line_no in 0..=max_y {
            let mut line = vec![format!("{}", line_no)];
            for column_no in 0..=max_x {
                let index = (column_no, line_no);
                line.push(if let Some(ptr) = cells.get(&index) {
                    format!("{}", ptr)
                } else {
                    String::from("")
                });
            }
            lines.push(line);
        }

        let mut table = Table::new();
        for line in lines {
            table.add_row(Row::new(
                line.iter().map(|s| Cell::new(s.as_str())).collect(),
            ));
        }
        // Print the table to stdout
        table.printstd();
    }
}
