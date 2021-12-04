pub fn input_data(problem_number: usize) -> String {
    let file_path = format!("input/{}.txt", problem_number);
    std::fs::read_to_string(file_path).unwrap()
}
