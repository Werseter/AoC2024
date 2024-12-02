use aoc_2024::day02::part1;
use aoc_2024::day02::part2;

fn main() {
    assert_eq!(part1::main("example_input.txt"), 2);
    println!("Day 02 Part 1: {:?}", part1::main("input.txt"));
    assert_eq!(part2::main("example_input.txt"), 4);
    println!("Day 02 Part 2: {:?}", part2::main("input.txt"));
}
