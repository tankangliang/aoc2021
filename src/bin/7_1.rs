use aoc2021::input_data;

fn main() {
    let crabs: Vec<usize> = input_data(7)
        .trim()
        .split(",")
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

    let max_crab = crabs.iter().max().unwrap();

    let mut crab_positions = vec![0; max_crab + 1];

    for crab in &crabs {
        crab_positions[*crab] += 1;
    }

    let mut sum_fuel_costs = vec![0; max_crab + 1];

    let mut crabs_in_hand = 0;
    let mut fuel_count = 0;

    for (i, crabs) in crab_positions.iter().enumerate() {
        fuel_count += crabs_in_hand;
        crabs_in_hand += crabs;
        sum_fuel_costs[i] += fuel_count;
    }

    crabs_in_hand = 0;
    fuel_count = 0;

    for (i, crabs) in crab_positions.iter().rev().enumerate() {
        let idx = max_crab - i;
        fuel_count += crabs_in_hand;
        crabs_in_hand += crabs;
        sum_fuel_costs[idx] += fuel_count;
    }

    println!("{}", sum_fuel_costs.iter().min().unwrap())
}
