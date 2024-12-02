fn is_valid_sequence(nums: &[i32]) -> bool {
    let mut is_ascending = false;
    let mut is_descending = false;

    for window in nums.windows(2) {
        let diff = window[1] - window[0];
        if !(1..4).contains(&diff.abs()) {
            return false;
        }
        match diff {
            diff if diff > 0 => is_ascending = true,
            diff if diff < 0 => is_descending = true,
            _ => {}
        }
        if is_ascending && is_descending {
            return false;
        }
    }
    true
}

fn generate_permutations(nums: &[i32]) -> impl Iterator<Item = Vec<i32>> + '_ {
    (0..nums.len()).flat_map(move |i| {
        let mut modified_nums = Vec::with_capacity(nums.len() - 1);
        modified_nums.extend_from_slice(&nums[..i]);
        modified_nums.extend_from_slice(&nums[i + 1..]);
        std::iter::once(modified_nums)
    })
}

#[must_use]
pub fn main(input_path: impl AsRef<std::path::Path>) -> usize {
    std::fs::read_to_string(input_path)
        .unwrap()
        .lines()
        .map(|line| {
            generate_permutations(
                &line
                    .split_whitespace()
                    .map(|level| level.parse().unwrap())
                    .collect::<Vec<i32>>(),
            )
            .any(|x| is_valid_sequence(&x))
        })
        .filter(|&x| x)
        .count()
}
