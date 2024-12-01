use aoc_2024::day01::part1;
use aoc_2024::day01::part2;

fn main() {
    assert_eq!(part1::main("example_input.txt"), 11);
    println!("Day 01 Part 1: {:?}", part1::main("input.txt"));
    assert_eq!(part2::main("example_input.txt"), 31);
    println!("Day 01 Part 2: {:?}", part2::main("input.txt"));
}
