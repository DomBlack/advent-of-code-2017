extern crate petgraph;

use petgraph::*;
use petgraph::prelude::UnGraphMap;

type Programs = UnGraphMap<u32, u32>;

#[cfg(not(test))]
fn main() {
    const INPUT: &str = include_str!("input.txt");

    let graph = to_graph(INPUT);

    println!("Part 1: {}", no_of_programs_in_group(&graph,0));
    println!("Part 2: {}", no_of_separate_groups(&graph));
}

/// Converts the input into a graph
fn to_graph(input: &str) -> Programs {
    let mut edges: Vec<(u32, u32)> = Vec::new();

    input.trim().lines().for_each( | line | {
        let parts = line.split(" <-> ").collect::<Vec<_>>();

        if parts.len() != 2 {
            panic!("Line malformed");
        }

        let node_id: u32 = parts[0].parse().expect("Unable to parse node ID");

        parts[1].split(", ")
            .map(| i | i.parse().expect("Unable to parse children ID"))
            .for_each( | other_id| edges.push((node_id, other_id)));
    });

    UnGraphMap::from_edges(edges)
}

fn no_of_programs_in_group(graph: &Programs, program: u32) -> u32 {
    use petgraph::prelude::Dfs;
    use petgraph::visit::Walker;

    Dfs::new(graph, program).iter(graph).count() as u32
}

fn no_of_separate_groups(graph: &Programs) -> u32 {
    algo::connected_components(graph) as u32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_examples() {
        const INPUT: &str = "0 <-> 2
1 <-> 1
2 <-> 0, 3, 4
3 <-> 2, 4
4 <-> 2, 3, 6
5 <-> 6
6 <-> 4, 5";

        let graph = to_graph(INPUT);

        assert_eq!(no_of_programs_in_group(&graph, 0), 6);
        assert_eq!(no_of_separate_groups(&graph), 2);
    }
}
