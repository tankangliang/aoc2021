use std::collections::{HashMap, HashSet};
use aoc2021::input_data;

fn process_input(input: String) -> HashMap<Vec<char>, usize> {

    let input = input.split_whitespace()
        .map(|s| s.chars().collect::<HashSet<char>>())
        .collect::<Vec<HashSet<char>>>();

    let mut vecs = vec![HashSet::new(); 10];

    let mut v235 = vec![];
    let mut v069 = vec![];

    for str in input {
        match str.len() {
            2 => vecs[1] = str,
            4 => vecs[4] = str,
            3 => vecs[7] = str,
            7 => vecs[8] = str,
            5 => v235.push(str),
            6 => v069.push(str),
            _ => unreachable!()
        }
    }

    let bd: HashSet<char> = vecs[4].difference(&vecs[1]).map(|c| *c).collect();
    let mut v23 = vec![];
    for str in v235 {
        if str.intersection(&bd).map(|c| *c).collect::<HashSet<char>>().len() == 2 {
            vecs[5] = str;
        } else {
            v23.push(str);
        }
    }

    let bdeg: HashSet<char> = vecs[8].difference(&vecs[7]).map(|c| *c).collect();
    let eg: HashSet<char> = bdeg.difference(&bd).map(|c| *c).collect();

    for str in v23 {
        if str.intersection(&eg).map(|c| *c).collect::<HashSet<char>>().len() == 2 {
            vecs[2] = str;
        } else {
            vecs[3] = str;
        }
    }

    let mut v69 = vec![];
    for str in v069 {
        if str.intersection(&bd).map(|c| *c).collect::<HashSet<char>>().len() == 2 {
            v69.push(str);
        } else {
            vecs[0] = str;
        }
    }
    for str in v69 {
        if str.intersection(&eg).map(|c| *c).collect::<HashSet<char>>().len() == 2 {
            vecs[6] = str;
        } else {
            vecs[9] = str;
        }
    }

    let mut map = HashMap::with_capacity(10);
    for (i, str) in vecs.into_iter().enumerate() {
        let mut s = str.into_iter().collect::<Vec<char>>();
        s.sort_unstable();
        map.insert(s, i);
    }

    map
}

fn main() {
    let content = input_data(8);
    // 1 -> 2
    // 4 -> 4
    // 7 -> 3
    // 8 -> 7
    // 2,3,5 -> 5
    // 5 -> 5 (can this 4-1 get b and d, check which 5 length has both)
    // 2,3 -> 8-7 gives me bdeg, minus (4-1) = eg. 2 has eg, 3 does not have eg
    // 0,6,9 -> 6
    // 0 -> without bd
    // 6 has eg, 9 does not

    let lines: Vec<(String, String)> = content.lines()
        .map(|line| line.split("|"))
        .map(|mut split| (split.next().unwrap().trim().to_string(),
                          split.next().unwrap().trim().to_string()))
        .collect();

    let mut count = 0;
    for (input, output) in lines {
        let map = process_input(input);
        let output = output.split_whitespace()
            .map(|s| s.chars().collect::<Vec<char>>())
            .map(|mut s| {s.sort_unstable(); s})
            .collect::<Vec<Vec<char>>>();

        let mut output_value = 0;
        for str in output {
            output_value *= 10;
            output_value += map.get(&str).unwrap();
        }
        count += output_value;
    }

    println!("{:?}", count)
}
