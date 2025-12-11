#![allow(unused_imports)]
use std::collections::HashMap;

use advent_derive::memoize;

use crate::utils::memoize::{AtomicStats, MemoizeStats, MemoizeStatsProvider};

fn parse(input: String) -> HashMap<String, Vec<String>> {
    let mut graph = HashMap::new();
    for line in input.lines() {
        let parts: Vec<&str> = line.split(": ").collect();
        let node = parts[0].to_string();
        let neighbors = parts[1].split_whitespace().map(|s| s.to_string()).collect();
        graph.insert(node, neighbors);
    }
    graph
}

#[memoize(key = (node, end))]
fn dfs(graph: &HashMap<String, Vec<String>>, node: &str, end: &str) -> u64 {
    if node == end {
        1
    } else {
        let Some(neighbors) = graph.get(node) else {
            return 0;
        };
        neighbors.iter().map(|n| dfs(graph, n, end)).sum()
    }
}

pub fn part1(input: String) -> String {
    let graph = parse(input);
    dfs(&graph, "you", "out").to_string()
}

pub fn part2(input: String) -> String {
    let graph = parse(input);
    let svr_to_dac = dfs(&graph, "svr", "dac");
    let dac_to_fft = dfs(&graph, "dac", "fft");
    let fft_to_out = dfs(&graph, "fft", "out");

    let svr_to_fft = dfs(&graph, "svr", "fft");
    let fft_to_dac = dfs(&graph, "fft", "dac");
    let dac_to_out = dfs(&graph, "dac", "out");

    let total_paths = svr_to_dac * dac_to_fft * fft_to_out + svr_to_fft * fft_to_dac * dac_to_out;

    total_paths.to_string()
}
