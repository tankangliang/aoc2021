use aoc2021::input_data;

fn main() {
    let content = input_data(9);
    let mut map = vec![];
    for line in content.lines() {
        let row: Vec<u32> = line.chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect();
        map.push(row);
    }

    let rows = map.len();
    let cols = map[0].len();

    let mut risk = 0;
    for (row_idx, row) in map.iter().enumerate() {
        for (col_idx, cell) in row.iter().enumerate() {
            let up = if row_idx == 0 { 10 } else {map[row_idx - 1][col_idx]};
            let down = if row_idx == rows - 1 { 10 } else {map[row_idx + 1][col_idx]};
            let right = if col_idx == cols - 1 { 10 } else {map[row_idx][col_idx+1]};
            let left = if col_idx == 0 { 10 } else {map[row_idx][col_idx-1]};
            let cell = *cell;
            if cell < up && cell < down && cell < right && cell < left {

                risk += cell + 1;
            }
        }
    }
    println!("{}", risk)
}
