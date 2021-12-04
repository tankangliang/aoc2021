fn main() {
    let contents = aoc2021::input_data(2)
        .lines()
        .map(|s| {
            let mut split = s.split_whitespace();
            let dir = split.next().unwrap().to_string();
            let amt = split.next().unwrap().parse::<i32>().unwrap();
            (dir, amt)
        })
        .collect::<Vec<(String, i32)>>();

    let (hor, dep, aim) = contents.iter()
        .fold((0i32, 0i32, 0i32), |(hor, dep, aim), (dir, amt)| {
            match dir.as_str() {
                "forward" => (hor + amt, dep + (aim * amt), aim),
                "down" => (hor, dep, aim + amt),
                _ => (hor, dep, aim - amt),
            }
        });

    println!("{}", hor * dep);
}
