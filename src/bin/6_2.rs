use std::collections::HashMap;
use aoc2021::input_data;

struct Simulator {
    dp: HashMap<usize, HashMap<usize, usize>>
}

impl Simulator {
    fn new() -> Self {
        Self {
            dp: HashMap::new()
        }
    }

    fn update(&mut self, lifetime: usize, days_left: usize, fishes: usize) {
        let map = self.dp
            .entry(lifetime)
            .or_insert(HashMap::new());

        map.insert(days_left, fishes);
    }

    fn simulate(&mut self, lifetime: usize, days_left: usize) -> usize {

        if days_left == 0 {
            return 1;
        }

        if let Some(map) = self.dp.get(&lifetime) {
            if let Some(ans) = map.get(&days_left) {
                return *ans;
            }
        }

        let ans = match (lifetime, days_left) {
            (0, days) => self.simulate(6, days - 1) + self.simulate(8, days - 1),
            (l, d) => self.simulate(l - 1, d - 1)
        };

        self.update(lifetime, days_left, ans);
        ans
    }
}



fn main() {
    let contents = input_data(6);

    let mut simulator = Simulator::new();

    let fishes: usize = contents
        .replace("\n", "")
        .split(",")
        .map(|life_str| life_str.parse::<usize>().unwrap())
        .map(|life| simulator.simulate(life, 256))
        .sum();

    println!("{:?}", fishes)
}
