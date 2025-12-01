#![allow(unused_imports)]

use crate::utils::memoize::{AtomicStats, MemoizeStats, MemoizeStatsProvider};
use advent_derive::memoize;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

pub fn part1(input: String) -> String {
    let lines = input.lines().collect::<Vec<_>>();
    lines
        .par_iter()
        .map(|line| {
            let (assignment, broken) = parse_input(line);
            count_possible_assignments(assignment, &broken)
        })
        .sum::<usize>()
        .to_string()
}

pub fn part2(input: String) -> String {
    let lines = input.lines().collect::<Vec<_>>();
    lines
        .par_iter()
        .map(|line| {
            let (assignment, broken) = parse_input(line);
            let (assignment, broken) = duplicate5(assignment, broken);
            count_possible_assignments(&assignment, &broken)
        })
        .sum::<usize>()
        .to_string()
}

pub fn duplicate5(assignemnt: &str, broken: Vec<usize>) -> (String, Vec<usize>) {
    let mut new_assignment = assignemnt.to_string();
    let mut new_broken = broken.clone();

    for _ in 0..4 {
        new_assignment.push_str(format!("?{}", assignemnt).as_str());
        new_broken.extend(broken.iter());
    }

    (new_assignment, new_broken)
}

pub fn parse_input(input: &str) -> (&str, Vec<usize>) {
    let mut split = input.split(' ');

    let assignment = split.next().unwrap();
    let broken = split
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    (assignment, broken)
}

pub fn count_possible_assignments(assignment: &str, groups: &[usize]) -> usize {
    count_assignments(assignment.to_string(), groups.to_vec(), 0, 0)
}

#[memoize(key = (pattern, groups, char_index, group_index))]
fn count_assignments(
    pattern: String,
    groups: Vec<usize>,
    char_index: usize,
    group_index: usize,
) -> usize {
    if group_index == groups.len() {
        if char_index >= pattern.len() || pattern[char_index..].chars().all(|c| c != '#') {
            return 1;
        } else {
            return 0;
        }
    }

    if char_index >= pattern.len() {
        return 0;
    }

    let mut count = 0;
    if pattern.chars().nth(char_index) != Some('#') {
        count += count_assignments(pattern.clone(), groups.clone(), char_index + 1, group_index);
    }

    let group_size = groups[group_index];
    let space_left = char_index + group_size <= pattern.len();
    let no_seperator = pattern
        [char_index..char_index.saturating_add(group_size).min(pattern.len())]
        .chars()
        .all(|c| c != '.');
    let seperator_after = space_left && pattern.chars().nth(char_index + group_size) != Some('#');

    if space_left && no_seperator && seperator_after {
        count += count_assignments(
            pattern.clone(),
            groups.clone(),
            char_index + group_size + 1,
            group_index + 1,
        );
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("???.### 1,1,3", 1)]
    #[case(".??..??...?##. 1,1,3", 4)]
    #[case("?#?#?#?#?#?#?#? 1,3,1,6", 1)]
    #[case("????.#...#... 4,1,1", 1)]
    #[case("????.######..#####. 1,6,5", 4)]
    #[case("?###???????? 3,2,1", 10)]
    fn test_count_possible_assignments(#[case] input: &str, #[case] expected: usize) {
        count_assignments_cache::clear();
        let (assignment, broken) = parse_input(input);
        let count = count_possible_assignments(assignment, &broken);

        assert_eq!(count, expected);
    }
}
