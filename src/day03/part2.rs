#[must_use]
pub fn main(input_path: impl AsRef<std::path::Path>) -> i32 {
    let mut mode = 1;
    regex::Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)")
        .unwrap()
        .captures_iter(std::fs::read_to_string(input_path).unwrap().as_str())
        .map(|m| match &m[0] {
            "do()" => (mode = 1, 0).1,
            "don't()" => (mode = 0, 0).1,
            _ => m[1].parse::<i32>().unwrap() * m[2].parse::<i32>().unwrap() * mode,
        })
        .sum()
}
