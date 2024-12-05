use aoc_2024::day04::part1;
use aoc_2024::day04::part2;

fn main() {
    assert_eq!(part1::main("example_input.txt"), 18);
    println!("Day 04 Part 1: {:?}", part1::main("input.txt"));
    assert_eq!(part2::main("example_input_2.txt"), 9);
    println!("Day 04 Part 2: {:?}", part2::main("input.txt"));
}
