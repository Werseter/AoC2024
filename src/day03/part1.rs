#[must_use]
pub fn main(input_path: impl AsRef<std::path::Path>) -> i32 {
    regex::Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)")
        .unwrap()
        .captures_iter(std::fs::read_to_string(input_path).unwrap().as_str())
        .map(|c| c.extract())
        .map(|(_, [a, b])| a.parse::<i32>().unwrap() * b.parse::<i32>().unwrap())
        .sum()
}
