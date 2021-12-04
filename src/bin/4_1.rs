use std::collections::HashMap;

struct Cell {
    num: i32,
    row: usize,
    col: usize,
    marked: bool
}

struct Board {
    map: HashMap<i32, Cell>,
    rows: [usize; 5],
    cols: [usize; 5]
}

impl Board {
    fn new(cells: &Vec<String>) -> Self {
        let mut map = HashMap::with_capacity(25);
        for (row, cell_row) in cells.iter().enumerate() {
            for (col, cell) in cell_row.trim().split_whitespace().enumerate() {
                let num: i32 = cell.parse().unwrap();
                map.insert(num, Cell {num, row, col, marked: false});
            }
        }

        Self {
            map,
            rows: [0; 5],
            cols: [0; 5],
        }
    }

    fn mark_number(&mut self, num: i32) -> Option<usize> {
        match self.map.get_mut(&num) {
            Some(v) => {
                v.marked = true;
                self.rows[v.row] += 1;
                self.cols[v.col] += 1;
                if self.rows[v.row] == 5 || self.cols[v.col] == 5 {
                    let mut sum = 0;
                    for (key, val) in self.map.iter() {
                        if !val.marked {
                            sum += val.num;
                        }
                    }

                    println!("sum: {}, num: {}", sum, num);
                    Some((sum * num) as usize)
                } else {
                    None
                }
            }
            None => None
        }
    }
}

fn main() {
    let contents: Vec<String> = aoc2021::input_data(4)
        .lines()
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect();

    let nums = contents.get(0).unwrap()
        .split(",")
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let mut boards = Vec::new();
    let mut cells = Vec::with_capacity(5);
    for cell_row in contents.into_iter().skip(1) {

        if cells.len() < 5 {
            cells.push(cell_row);
        } else {
            boards.push(Board::new(&cells));
            cells.clear();
            cells.push(cell_row);
        }
    }

    if cells.len() == 5 {
        boards.push(Board::new(&cells));
    }

    for num in nums {
        for board in boards.iter_mut() {
            if let Some(ans) = board.mark_number(num) {
                println!("{}", ans);
                return;
            }
        }
    }
    println!("{}", boards.len());
}
