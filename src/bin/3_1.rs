fn main() {
    let contents: Vec<i32> = aoc2021::input_data(3)
        .lines()
        .map(|s| i32::from_str_radix(s, 2).unwrap())
        .collect();

    let mut bits = [0; 12];
    let len = contents.len();

    for mut i in contents {
        for bit in (0usize..12).rev() {
            bits[bit] += (i & 1);
            i >>= 1;
        }
    }

    let mut gamma = 0;
    let mut epsilon = 0;

    for bit_count in bits {
        gamma <<= 1;
        epsilon <<= 1;

        if bit_count > (len / 2) as i32 {
            gamma += 1;
        } else {
            epsilon += 1;
        }
    }

    println!("{}", gamma * epsilon);
}
