use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;
use aoc2021::input_data;

const DIRECTIONS: [(i32, i32); 4] = [
    (-1 ,0), // TOP
    (1, 0), // BOTTOM,
    (0, -1), // LEFT,
    (0, 1) // RIGHT
];

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct Cell {
    row: usize,
    col: usize,
    risk_so_far: usize,
    parent: (usize, usize)
}

impl PartialOrd<Self> for Cell {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.risk_so_far.partial_cmp(&other.risk_so_far)
    }
}

impl Ord for Cell {
    fn cmp(&self, other: &Self) -> Ordering {
        self.risk_so_far.cmp(&other.risk_so_far)
    }
}

fn main() {
    let content = input_data(15);
    let mut map = content.lines()
        .map(|r| r
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect::<Vec<usize>>())
        .collect::<Vec<Vec<usize>>>();

    let rows = map.len();
    let cols = map[0].len();

    let mut parents = vec![vec![(0,0); cols]; rows];
    let mut visited = vec![vec![false; cols]; rows];
    let mut heap = BinaryHeap::new();
    let origin = Cell {
        row: 0,
        col: 0,
        risk_so_far: 0,
        parent: (0, 0)
    };

    heap.push(Reverse(origin));

    while let Some(cell) = heap.pop() {

        let cell = cell.0;
        if visited[cell.row][cell.col] {
            continue;
        }
        let risk_so_far = cell.risk_so_far + map[cell.row][cell.col];
        visited[cell.row][cell.col] = true;
        parents[cell.row][cell.col] = cell.parent;

        if cell.row == rows - 1 && cell.col == cols - 1 {
            break;
        }


        for (row_diff, col_diff) in DIRECTIONS {
            let new_row = cell.row as i32 + row_diff;
            let new_col = cell.col as i32 + col_diff;

            if new_row < 0 || new_row >= rows as i32 || new_col < 0 || new_col >= cols as i32 {
                continue;
            }

            let new_row = new_row as usize;
            let new_col = new_col as usize;
            heap.push(Reverse(Cell {
                row: new_row,
                col: new_col,
                risk_so_far,
                parent: (cell.row, cell.col)
            }))
        }
    }

    let mut total_risk = map[rows-1][cols-1];
    let mut next_path = parents[rows-1][cols-1];
    while next_path != (0, 0) {

        let (row, col) = next_path;
        total_risk += map[row][col];
        next_path = parents[row][col];
    }

    println!("{}", total_risk);
}
