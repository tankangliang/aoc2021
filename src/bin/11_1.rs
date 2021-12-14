use aoc2021::input_data;

const DIRECTIONS: [(i32, i32); 8] = [
    (-1, -1), (-1, 0), (-1, 1),
    (0, -1),          (0, 1),
    (1, -1), (1, 0), (1, 1)
];

struct Grid {
    grid: [[u32; 10]; 10],
    flashed: [[bool; 10]; 10],
    flashes: usize
}

impl Grid {
    fn new(s: String) -> Self {
        let mut grid = [[0; 10]; 10];
        for (row_idx, row) in s.lines().enumerate() {
            for (col_idx, c) in row.chars().enumerate() {
                grid[row_idx][col_idx] = c.to_digit(10).unwrap();
            }
        }

        let flashed = [[false; 10]; 10];
        let flashes = 0;

        Self {
            grid,
            flashed,
            flashes
        }
    }

    fn increment(&mut self, row: i32, col: i32) {
        if row >= 0 && row <= 9 && col >= 0 && col <= 9 {
            let row = row as usize;
            let col = col as usize;

            if self.flashed[row][col] {
                return;
            }

            self.grid[row][col] += 1;
            if self.grid[row][col] > 9 {
                self.flash(row, col)
            }
        }
    }

    fn step(&mut self) {
        self.flashed = [[false; 10]; 10];

        for row in 0..10 {
            for col in 0..10 {
                self.increment(row, col);
            }
        }

    }

    fn flash(&mut self, row: usize, col: usize) {
        if self.flashed[row][col] {

        } else {
            self.grid[row][col] = 0;
            self.flashes += 1;
            self.flashed[row][col] = true;
            for (x, y) in DIRECTIONS {
                let row = row as i32;
                let col = col as i32;
                self.increment(row + x, col + y);
            }
        }
    }

    fn print_grid(&self, step_no: usize) {
        println!("Step {}", step_no);
        for row in &self.grid {
            println!("{:?}", row);
        }
        println!()
    }
}

fn main() {
    let content = input_data(11);
    let mut grid = Grid::new(content);

    for step in 0..100 {
        grid.step();
        // grid.print_grid(step);
    }
    
    println!("{}", grid.flashes)
}
