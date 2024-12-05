#[must_use]
pub fn main(input_path: impl AsRef<std::path::Path>) -> usize {
    let graph = std::fs::read_to_string(input_path)
        .unwrap()
        .lines()
        .zip(0..)
        .fold(
            std::collections::HashMap::<char, std::collections::HashSet<_>>::new(),
            |mut graph, (line, i)| {
                line.chars().zip(0..).for_each(|(char, j)| {
                    graph.entry(char).or_default().insert((i, j));
                });
                graph
            },
        );
    (-1..=1)
        .flat_map(|x| (-1..=1).map(move |y| (x, y)))
        .filter(|&pos| pos != (0, 0))
        .flat_map(|(x_diff, y_diff)| {
            graph
                .get(&'X')
                .unwrap()
                .iter()
                .map(move |(x, y)| (x + x_diff, y + y_diff))
                .filter(|idx| graph.get(&'M').unwrap().contains(idx))
                .map(move |(x, y)| (x + x_diff, y + y_diff))
                .filter(|idx| graph.get(&'A').unwrap().contains(idx))
                .map(move |(x, y)| (x + x_diff, y + y_diff))
                .filter(|idx| graph.get(&'S').unwrap().contains(idx))
        })
        .count()
}
