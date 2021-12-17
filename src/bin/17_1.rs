use std::convert::TryInto;
use aoc2021::input_data;

struct Point {
    x: i32,
    y: i32
}

struct Probe {
    target_bottom_left: Point,
    target_top_right: Point,
    location: Point,
    x_velocity: i32,
    y_velocity: i32,
    highest_y: i32,
}

impl Probe {
    fn new(min_x: i32, min_y: i32, max_x: i32, max_y: i32, x_vel: i32, y_vel: i32) -> Self {
        Self {
            target_bottom_left: Point{x: min_x, y: min_y},
            target_top_right: Point{x: max_x, y: max_y},
            location: Point{ x:0, y:0},
            x_velocity: x_vel,
            y_velocity: y_vel,
            highest_y: 0
        }
    }

    fn simulate(&mut self) -> (bool, i32) {
        loop {
            self.location.x += self.x_velocity;
            self.location.y += self.y_velocity;
            self.highest_y = self.highest_y.max(self.location.y);
            if self.x_velocity > 0 {
                self.x_velocity -= 1;
            } else if self.x_velocity < 0 {
                self.x_velocity += 1;
            }

            self.y_velocity -= 1;

            let x = self.location.x;
            let y = self.location.y;

            if x > self.target_top_right.x || (x < self.target_bottom_left.x && self.x_velocity <= 0) {
                break;
            }

            if y < self.target_bottom_left.y && self.y_velocity <= 0 {
                break;
            }

            if x <= self.target_top_right.x
                && x >= self.target_bottom_left.x
                && y <= self.target_top_right.y
                && y >= self.target_bottom_left.y {
                return (true, self.highest_y)
            }
        }
        (false, 0)
    }
}

fn main() {
    let content = input_data(17);

    let mut input = content.trim()
        .strip_prefix("target area: ")
        .unwrap()
        .split(",");

    let mut xs = input.next().unwrap().trim()
        .strip_prefix("x=")
        .unwrap().split("..");

    let min_x = xs.next().unwrap().parse::<i32>().unwrap();
    let max_x = xs.next().unwrap().parse::<i32>().unwrap();

    let mut ys = input.next().unwrap().trim()
        .strip_prefix("y=")
        .unwrap().split("..");

    let min_y = ys.next().unwrap().parse::<i32>().unwrap();
    let max_y = ys.next().unwrap().parse::<i32>().unwrap();

    let mut highest_y = 0;

    for x in 0..100 {
        for y in 0..100 {
            let mut probe = Probe::new(min_x, min_y, max_x, max_y, x, y);
            let (reached, top) = probe.simulate();
            if reached {
                highest_y = highest_y.max(top)
            }
        }
    }


    println!("{}", highest_y);
}
