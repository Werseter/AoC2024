use aoc_2024::day03::part1;
use aoc_2024::day03::part2;

fn main() {
    assert_eq!(part1::main("example_input.txt"), 161);
    println!("Day 03 Part 1: {:?}", part1::main("input.txt"));
    assert_eq!(part2::main("example_input_2.txt"), 48);
    println!("Day 03 Part 2: {:?}", part2::main("input.txt"));
}
