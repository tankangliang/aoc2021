use std::collections::HashMap;
use aoc2021::input_data;

fn merge(mut map1: HashMap<char, u64>, map2: HashMap<char, u64>) -> HashMap<char, u64> {
    for (k, v) in map2 {
        *map1.entry(k).or_default() += v;
    }
    map1
}

fn new_map(a: char, b: char) -> HashMap<char, u64> {
    let mut map = HashMap::new();
    *map.entry(a).or_default() += 1;
    *map.entry(b).or_default() += 1;
    map
}

fn move_step(a: char,
             b: char,
             rules: &HashMap<(char, char), char>,
             steps: usize,
             dp: &mut HashMap<(char, char, usize), HashMap<char, u64>>) -> HashMap<char, u64> {

    if steps == 0 {
        return new_map(a, b);
    };

    if let Some(map) = dp.get(&(a,b,steps)) {
        return map.clone();
    }

    match rules.get(&(a,b)) {
        Some(val) => {
            let val = *val;
            let map1 = move_step(a, val, rules, steps - 1, dp);
            let map2  = move_step(val, b, rules, steps - 1, dp);
            let mut merged = merge(map1, map2);
            *merged.entry(val).or_default() -= 1;
            dp.insert((a, b, steps), merged.clone());
            merged
        },
        None => {
            new_map(a, b)
        },
    }
}

fn main() {
    let STEPS = 40;
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


    let mut dp = HashMap::new();

    let mut windows = initial.windows(2);
    let first = windows.next().unwrap();
    let mut count = move_step(first[0], first[1], &rules, STEPS, &mut dp);
    for window in windows {
        let map = move_step(window[0], window[1], &rules, STEPS, &mut dp);
        count = merge(count, map);
        *count.entry(window[0]).or_default() -= 1;
    }

    let max = *count.values().max().unwrap();
    let min = *count.values().min().unwrap();

    println!("{}", max - min);
}
