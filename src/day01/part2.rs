#[must_use]
pub fn main(input_path: impl AsRef<std::path::Path>) -> i32 {
    let input = std::fs::read_to_string(input_path).unwrap();
    let input_size = input.lines().count();
    let mut first_column: Vec<i32> = Vec::with_capacity(input_size);
    let mut second_column: std::collections::HashMap<i32, i32> = std::collections::HashMap::new();
    for (a, b) in input.lines().map(|line| line.split_once(' ').unwrap()) {
        first_column.push(a.parse::<i32>().unwrap());
        *second_column
            .entry(b.trim().parse::<i32>().unwrap())
            .or_insert(0) += 1;
    }
    first_column.sort_unstable();
    let result = first_column
        .iter()
        .map(|a| a * second_column.get(a).unwrap_or(&0))
        .sum();
    result
}
