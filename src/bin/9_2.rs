use std::collections::{HashMap, HashSet};
use aoc2021::input_data;

struct Map {
    map: Vec<Vec<u32>>,
    rows: usize,
    cols: usize,
    dp: Vec<Vec<Option<(usize, usize)>>>,
}

impl Map {
    fn new(map: Vec<Vec<u32>>) -> Self {
        let rows = map.len();
        let cols = map[0].len();
        let dp = vec![vec![None; cols]; rows];
        Self {
            map,
            rows,
            cols,
            dp
        }
    }

    fn find_lowest_point(&mut self, row: usize, col: usize) -> (usize, usize) {
        let me = self.map[row][col];
        if me == 9 {
            (row, col)
        } else {
            if let Some(prev_ans) = self.dp[row][col] {
                prev_ans
            } else {
                let up = if row == 0 { 10 } else {self.map[row - 1][col]};
                let down = if row == self.rows - 1 { 10 } else {self.map[row + 1][col]};
                let right = if col == self.cols - 1 { 10 } else {self.map[row][col+1]};
                let left = if col == 0 { 10 } else {self.map[row][col-1]};

                let mut endpoints = vec![];
                if me > up {
                    endpoints.push(self.find_lowest_point(row - 1, col))
                }
                if me > down {
                    endpoints.push(self.find_lowest_point(row + 1, col))
                }
                if me > right {
                    endpoints.push(self.find_lowest_point(row, col + 1))
                }
                if me > left {
                    endpoints.push(self.find_lowest_point(row, col - 1))
                }

                use std::iter::FromIterator;
                let set: HashSet<(usize, usize)> = HashSet::from_iter(endpoints);
                if set.len() == 1 {
                    let ans = set.iter().last().unwrap();
                    self.dp[row][col] = Some(*ans);
                    *ans
                } else {
                    self.dp[row][col] = Some((row, col));
                    (row, col)
                }
            }

        }
    }
}

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

    let mut counter = HashMap::new();
    let mut map = Map::new(map);

    for row in 0..rows {
        for col in 0..cols {
            let pt = map.find_lowest_point(row, col);
            let entry = counter.entry(pt).or_insert(0);
            *entry += 1;
        }
    }

    let mut v: Vec<i32> = counter.into_values().collect();
    v.sort_by(|a, b| b.cmp(a));

    println!("{:#?}", v[0] * v[1] * v[2]);

}
