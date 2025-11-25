use std::cmp::min;

use itertools::Itertools;

pub fn part1(input: String) -> String {
    let grids = parse_input(&input);

    grids
        .iter()
        .map(|grid| grid.find_mirror_line(0))
        .sum::<usize>()
        .to_string()
}

pub fn part2(input: String) -> String {
    let grids = parse_input(&input);

    grids
        .iter()
        .map(|grid| grid.find_mirror_line(1))
        .sum::<usize>()
        .to_string()
}

#[derive(Debug)]
struct Grid {
    collumns: Vec<String>,
    rows: Vec<String>,
}

fn parse_input(input: &str) -> Vec<Grid> {
    input.split("\n\n").map(|grid| parse_grid(grid)).collect()
}

fn parse_grid(grid: &str) -> Grid {
    let rows: Vec<String> = grid.lines().map(|row| row.to_string()).collect();

    let collumns = (0..rows[0].len())
        .map(|i| {
            rows.iter()
                .map(|row| row.chars().nth(i).unwrap())
                .collect::<String>()
        })
        .collect();

    Grid { rows, collumns }
}

impl Grid {
    fn find_mirror_line(&self, smudges: usize) -> usize {
        if let Some(i) = find_mirror_line(&self.rows, smudges) {
            (i + 1) * 100
        } else {
            find_mirror_line(&self.collumns, smudges).unwrap() + 1
        }
    }
}

fn find_mirror_line(lines: &[String], smudges: usize) -> Option<usize> {
    lines
        .iter()
        .enumerate()
        .tuple_windows()
        .filter_map(|((i, line1), (_, line2))| {
            let mut smudges = smudges;
            if compare_with_smudge(line1, line2, &mut smudges) {
                Some(i)
            } else {
                None
            }
        })
        .filter(|i| validate_miror_line(lines, *i, smudges))
        .next()
}

fn validate_miror_line(lines: &[String], line: usize, smudges: usize) -> bool {
    let mut smudges = smudges;
    for i in 0..=min(line, lines.len() - line - 2) {
        if !compare_with_smudge(&lines[line - i], &lines[line + i + 1], &mut smudges) {
            return false;
        }
    }

    smudges == 0
}

fn compare_with_smudge(line1: &str, line2: &str, smudges_left: &mut usize) -> bool {
    if *smudges_left == 0 {
        return line1 == line2;
    }
    let smudges = line1
        .chars()
        .zip(line2.chars())
        .filter(|(c1, c2)| c1 != c2)
        .count();

    if smudges > *smudges_left {
        return false;
    }

    *smudges_left -= smudges;
    true
}
