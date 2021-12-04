use std::ops::Shr;

fn get_rating(initial_bits: &Vec<i32>, look_for_one: bool) -> i32 {
    let mut rating;
    let mut current: Vec<i32> = initial_bits.clone();
    let mut ones = Vec::new();
    let mut zeros = Vec::new();
    let mut shift_amount = 11;

    loop {
        for num in current {
            let bit_at_position = (num.checked_shr(shift_amount).unwrap_or(0)) & 1;
            if bit_at_position == 1 {
                ones.push(num);
            } else {
                zeros.push(num);
            }
        }

        match (ones.len() >= zeros.len(), look_for_one) {
            (true, true) => current = ones.clone(),
            (true, false) => current = zeros.clone(),
            (false, true) => current = zeros.clone(),
            (false, false) => current = ones.clone()
        }

        if current.len() == 1 {
            rating = *current.get(0).unwrap();
            break;
        }

        ones.clear();
        zeros.clear();
        shift_amount -= 1;
    }

    rating
}

fn main() {
    let contents: Vec<i32> = aoc2021::input_data(3)
        .lines()
        .map(|s| i32::from_str_radix(s, 2).unwrap())
        .collect();

    let oxygen_generator_rating = get_rating(&contents, true);
    let co2_scrubber_rating = get_rating(&contents, false);

    println!("{}", oxygen_generator_rating * co2_scrubber_rating);
}
