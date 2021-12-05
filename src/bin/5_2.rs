use aoc2021::input_data;

#[derive(Debug)]
struct Point(usize, usize);

impl Point {
    fn from_pt_str(s: &str) -> Self {
        // 276,861
        let mut split = s.trim().split(",");

        let x = split.next()
            .map(|x_str| x_str.parse::<usize>().unwrap())
            .unwrap();

        let y = split.next()
            .map(|y_str| y_str.parse::<usize>().unwrap())
            .unwrap();

        Self(x, y)
    }
}

#[derive(Debug)]
struct Line {
    from: Point,
    to: Point
}

impl Line {
    fn from_line_str(s: &str) -> Self {
        // 276,861 -> 657,480
        let mut split = s.split("->");

        Self {
            from: Point::from_pt_str(split.next().unwrap()),
            to: Point::from_pt_str(split.next().unwrap())
        }
    }

    fn is_horizontal_or_vertical(&self) -> bool {
        self.from.0 == self.to.0 || self.from.1 == self.to.1
    }
}

fn main() {
    // Parse lines
    // Filter out horizontal and vertical lines
    let lines: Vec<Line> = input_data(5)
        .lines()
        .map(|line| Line::from_line_str(line))
        .collect();


    // Get max x and y to form diagram
    let mut max_x = 0;
    let mut max_y = 0;

    for line in &lines {
        max_x = max_x.max(line.from.0).max(line.to.0);
        max_y = max_y.max(line.from.1).max(line.to.1);
    }

    // Mark positions where line starts and ends
    let mut positions = vec![vec![0; max_y + 1]; max_x + 1];
    for line in &lines {
        if line.from.0 == line.to.0 {
            // we go from smaller to larger
            let from_y = line.from.1.min(line.to.1);
            let to_y = line.from.1.max(line.to.1);

            for y in from_y..=to_y {
                positions[line.from.0][y] += 1;
            }
        } else if line.from.1 == line.to.1 {
            // we go from smaller to larger
            let from_x = line.from.0.min(line.to.0);
            let to_x = line.from.0.max(line.to.0);

            for x in from_x..=to_x {
                positions[x][line.from.1] += 1;
            }
        } else {
            let mut start_x = line.from.0;
            let mut start_y = line.from.1;

            while start_x != line.to.0 && start_y != line.to.1 {
                positions[start_x][start_y] += 1;

                if start_x < line.to.0 {
                    start_x += 1;
                } else {
                    start_x -= 1;
                }

                if start_y < line.to.1 {
                    start_y += 1;
                } else {
                    start_y -= 1;
                }
            }

            positions[line.to.0][line.to.1] += 1;
        }
    }

    let mut count = 0;
    for row in positions {
        // println!("{:?}", row);
        for cell in row {
            if cell > 1 {
                count += 1;
            }
        }
    }

    println!("{}", count);
}
