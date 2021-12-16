use std::collections::HashMap;
use aoc2021::input_data;

fn main() {
    let content = input_data(14);
    let mut lines = content.lines();
    let mut initial = lines.next().unwrap().chars().collect::<Vec<char>>();

    lines.next().unwrap();

    let mut rules = HashMap::new();
    for rule in lines {
        let mut split = rule.split("->");

        let mut key = split.next().unwrap().trim().chars().take(2);
        let key = (key.next().unwrap(), key.next().unwrap());
        let mut value = split.next().unwrap().trim().chars().take(1);
        let value = value.next().unwrap();

        rules.insert(key, value);
    }


    for _ in 1..=10 {
        let mut new_strand = vec![];
        let mut windows = initial.windows(2);

        let first = windows.next().unwrap();
        new_strand.push(first[0]);
        let key = (first[0], first[1]);
        match rules.get(&key) {
            Some(val) => new_strand.push(*val),
            None => {},
        }
        new_strand.push(first[1]);

        for window in windows {
            let key = (window[0], window[1]);
            match rules.get(&key) {
                Some(val) => new_strand.push(*val),
                None => {},
            }
            new_strand.push(window[1]);
        }

        initial = new_strand;
    }

    let mut count: HashMap<char, u32> = HashMap::new();
    for c in initial {
        *count.entry(c).or_default() += 1;
    }

    let max = *count.values().max().unwrap();
    let min = *count.values().min().unwrap();

    println!("{}", max - min);
}
