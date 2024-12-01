#[must_use]
pub fn main(input_path: impl AsRef<std::path::Path>) -> i32 {
    let input = std::fs::read_to_string(input_path).unwrap();
    let input_size = input.lines().count();
    let mut first_column: Vec<i32> = Vec::with_capacity(input_size);
    let mut second_column: Vec<i32> = Vec::with_capacity(input_size);
    for (a, b) in input.lines().map(|line| line.split_once(' ').unwrap()) {
        first_column.push(a.parse::<i32>().unwrap());
        second_column.push(b.trim().parse::<i32>().unwrap());
    }
    first_column.sort_unstable();
    second_column.sort_unstable();
    let result = first_column
        .iter()
        .zip(second_column.iter())
        .map(|(a, b)| (a - b).abs())
        .sum();
    result
}
