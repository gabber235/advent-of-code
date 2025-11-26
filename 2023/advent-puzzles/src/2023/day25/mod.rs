use indicatif::{ParallelProgressIterator, ProgressStyle};
use itertools::Itertools;
use nom::{character::complete::alpha1, multi::separated_list1, IResult};
use petgraph::graph::{NodeIndex, UnGraph};
use petgraph::visit::{EdgeRef, IntoNodeReferences};
use rayon::iter::{ParallelBridge, ParallelIterator};
use std::collections::{HashMap, HashSet, VecDeque};

pub fn part1(input: String) -> String {
    let edges = input
        .lines()
        .flat_map(|line| parse_line(line).unwrap().1)
        .collect::<Vec<_>>();

    let nodes = edges
        .iter()
        .flat_map(|(source, target)| vec![source, target])
        .collect::<HashSet<_>>();

    let mut graph = UnGraph::<&str, ()>::new_undirected();

    let nodes = nodes
        .iter()
        .map(|node| (**node, graph.add_node(node)))
        .collect::<HashMap<_, _>>();

    for (source, target) in edges {
        graph.add_edge(nodes[&source], nodes[&target], ());
    }

    let frequencies = calculate_edge_frequencies(&graph);

    let cut_edges = find_cut_edges(&frequencies, 3);
    let partition_size = calculate_partition_size(&graph, &cut_edges);

    format!("{:?}", partition_size)
}

fn calculate_edge_frequencies(graph: &UnGraph<&str, ()>) -> HashMap<(usize, usize), usize> {
    let mut freq = HashMap::new();

    for start in graph.node_indices() {
        let mut seen = HashSet::new();
        let mut todo = VecDeque::new();
        seen.insert(start);
        todo.push_back(start);

        while let Some(pos) = todo.pop_front() {
            for edge in graph.edges(pos) {
                if seen.contains(&edge.target()) {
                    continue;
                }
                let edge_key = edge_key(edge);
                let count = freq.entry(edge_key).or_insert(0);
                *count += 1;

                seen.insert(edge.target());
                todo.push_back(edge.target());
            }
        }
    }

    freq
}

fn find_cut_edges(
    frequencies: &HashMap<(usize, usize), usize>,
    cut_size: usize,
) -> Vec<(usize, usize)> {
    let mut order: Vec<_> = frequencies.iter().collect();
    order.sort_unstable_by(|a, b| b.1.cmp(a.1));
    order
        .iter()
        .take(cut_size)
        .map(|(edge, _)| **edge)
        .collect()
}

fn calculate_partition_size(graph: &UnGraph<&str, ()>, cut_edges: &[(usize, usize)]) -> usize {
    let start = graph.node_indices().next().unwrap();
    let mut size = 1;
    let mut todo = VecDeque::new();
    todo.push_back(start);
    let mut seen = HashSet::new();
    seen.insert(start);

    while let Some(pos) = todo.pop_front() {
        for edge in graph.edges(pos) {
            let edge_key = edge_key(edge);

            if cut_edges.contains(&edge_key) {
                continue;
            }

            if seen.insert(edge.target()) {
                size += 1;
                todo.push_back(edge.target());
            }
        }
    }

    size * (graph.node_count() - size)
}

fn edge_key(edge: petgraph::graph::EdgeReference<()>) -> (usize, usize) {
    if edge.source() < edge.target() {
        (edge.source().index(), edge.target().index())
    } else {
        (edge.target().index(), edge.source().index())
    }
}

pub fn part2(input: String) -> String {
    todo!()
}

fn parse_line(input: &str) -> IResult<&str, Vec<(&str, &str)>> {
    let (input, head) = alpha1(input)?;
    let (input, _) = nom::bytes::complete::tag(": ")(input)?;

    let (input, targets) = separated_list1(nom::bytes::complete::tag(" "), alpha1)(input)?;

    let edges = targets
        .iter()
        .map(|target| (head, *target))
        .collect::<Vec<_>>();

    Ok((input, edges))
}
