use aoc2021::input_data;

fn main() {
    let count = input_data(1).lines()
        .map(|s| s.parse::<usize>().unwrap())
        .fold((0, (0,0,0)), |(count, (a,b,c)), curr| {
            match (a,b,c) {
                (0, _, _) => (count, (curr, b, c)),
                (_, 0, _) => (count, (a, curr, c)),
                (_, _, 0) => (count, (a, b, curr)),
                (a, b, c) => {
                    if a + b + c < b + c + curr {
                        (count + 1, (b, c, curr))
                    } else {
                        (count, (b, c, curr))
                    }
                }
            }
        })
        .0;


    println!("{}", count)
}
