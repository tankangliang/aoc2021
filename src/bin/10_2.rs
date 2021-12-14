use std::collections::HashMap;
use aoc2021::input_data;

fn parse_line(s: &str) -> usize {
    let mapping: HashMap<char, char> = HashMap::from([
        ('(', ')'),
        ('[', ']'),
        ('{', '}'),
        ('<', '>'),
    ]);

    let mut stack = Vec::with_capacity(s.len());

    for c in s.chars() {
        match c {
            '(' | '[' | '{' | '<' => stack.push(c),
            _ => {
                match stack.pop() {
                    Some(popped) => {
                        let pairing = *mapping.get(&popped).unwrap();
                        if c != pairing {
                            return 0;
                        }
                    },
                    None => {}
                }
            }
        }
    }

    let mut score = 0;
    for c in stack.iter().rev() {
        score *= 5;
        score += match mapping.get(c).unwrap() {
            ')' => 1,
            ']' => 2,
            '}' => 3,
            '>' => 4,
            _ => 0
        }
    }
    score
}

fn main() {
    let mut content = input_data(10)
        .lines()
        .map(parse_line)
        .filter(|score| *score != 0)
        .collect::<Vec<usize>>();

    content.sort_unstable();
    println!("{}", content.get(content.len() / 2).unwrap())
}
