use aoc2021::input_data;

fn main() {
    let content = input_data(8);
    // 1 -> 2
    // 4 -> 4
    // 7 -> 3
    // 8 -> 7

    let lines: Vec<(String, String)> = content.lines()
        .map(|line| line.split("|"))
        .map(|mut split| (split.next().unwrap().trim().to_string(),
                          split.next().unwrap().trim().to_string()))
        .collect();

    let count = lines.iter()
        .flat_map(|(_, output)| output
            .split_whitespace()
            .collect::<Vec<&str>>())
        .fold(0, |acc, output| {
            match output.len() {
                2 | 4 | 3 | 7 => acc + 1,
                _ => acc
            }
        });

    println!("{:?}", count)
}
