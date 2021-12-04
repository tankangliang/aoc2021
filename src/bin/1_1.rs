use aoc2021::input_data;

fn main() {
    let count = input_data(1).lines()
        .map(|s| s.parse::<usize>().unwrap())
        .fold((-1, 0), |(count, prev), curr| {
            if curr > prev {
                (count + 1, curr)
            } else {
                (count, curr)
            }
        })
        .0;


    println!("{}", count)
}
