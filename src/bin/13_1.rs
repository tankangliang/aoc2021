use std::collections::HashSet;
use aoc2021::input_data;

enum Fold {
    X(usize),
    Y(usize)
}

type Dot = (usize, usize);

struct Grid {
    dots: HashSet<Dot>,
}

impl Grid {
    fn fold(self, fold: &Fold) -> Grid {
        match fold {
            Fold::X(position) => self.fold_x(*position),
            Fold::Y(position) => self.fold_y(*position)
        }
    }

    fn fold_x(self, position: usize) -> Grid {
        let mut new_dots = HashSet::new();
        for (x, y) in self.dots {
            match x {
                _ if x < position => {new_dots.insert((x,y));},
                _ if x > position => {
                    let offset_from_position = x - position;
                    let new_x = position - offset_from_position;
                    new_dots.insert((new_x, y));
                },
                _ => {}
            }
        }

        Self {
            dots: new_dots
        }
    }

    fn fold_y(self, position: usize) -> Grid {
        let mut new_dots = HashSet::new();
        for (x, y) in self.dots {
            match y {
                _ if y < position => {new_dots.insert((x,y));},
                _ if y > position => {
                    let offset_from_position = y - position;
                    let new_y = position - offset_from_position;
                    new_dots.insert((x, new_y));
                },
                _ => {}
            }
        }

        Self {
            dots: new_dots
        }
    }
}

fn main() {
    let content = input_data(13);

    let mut parse_dots = true;
    let mut dots: HashSet<Dot> = HashSet::new();
    let mut folds = vec![];

    for line in content.lines() {
        if line.is_empty() {
            parse_dots = false;
            continue;
        }

        if parse_dots {
            let mut split = line.split(",");
            let mut x = split.next().unwrap().parse::<usize>().unwrap();
            let mut y = split.next().unwrap().parse::<usize>().unwrap();
            dots.insert((x, y));
        } else {
            let remainder = line.strip_prefix("fold along ").unwrap();
            let mut split = remainder.split("=");
            let direction = split.next().unwrap();
            let position = split.next().unwrap().parse::<usize>().unwrap();
            let fold = match direction {
                "x" => Fold::X(position),
                _ => Fold::Y(position)
            };
            folds.push(fold);
        }
    }

    let mut grid = Grid { dots };

    let first_fold = folds.first().unwrap();
    grid = grid.fold(first_fold);
    
    println!("{}", grid.dots.len());
}
