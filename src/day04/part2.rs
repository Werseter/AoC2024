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
    let m_set = graph.get(&'M').unwrap();
    let a_set = graph.get(&'A').unwrap();
    let s_set = graph.get(&'S').unwrap();

    [(-1, -1), (-1, 1), (1, -1), (1, 1)]
        .iter()
        .flat_map(|&(x_diff, y_diff)| {
            a_set.iter().filter(move |&&(x, y)| {
                m_set.contains(&(x + x_diff, y + y_diff))
                    && s_set.contains(&(x - x_diff, y - y_diff))
                    && (m_set.contains(&(x - x_diff, y + y_diff))
                        && s_set.contains(&(x + x_diff, y - y_diff))
                        || (m_set.contains(&(x + x_diff, y - y_diff))
                            && s_set.contains(&(x - x_diff, y + y_diff))))
            })
        })
        .count()
        / 2
}
