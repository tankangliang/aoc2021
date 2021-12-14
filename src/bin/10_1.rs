use std::collections::HashMap;
use aoc2021::input_data;

fn score(c: char) -> usize {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0
    }
}

fn parse_line(s: &str) -> usize {

    let mapping = HashMap::from([
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
                            return score(c);
                        }
                    },
                    None => {}
                }
            }
        }
    }

    0
}

fn main() {
    let content: usize = input_data(10)
        .lines()
        .map(parse_line)
        .sum();
    
    println!("{}", content)
}
