use itertools::Itertools;

use crate::utils::parsing::{blocks, numbers};

pub fn part1(input: String) -> String {
    let blocks = blocks(&input);
    let ranges = blocks[0]
        .lines()
        .map(|line| {
            let mut parts = line.split('-');
            let start = parts.next().unwrap().parse::<i64>().unwrap();
            let end = parts.next().unwrap().parse::<i64>().unwrap();
            start..=end
        })
        .collect::<Vec<_>>();

    let ids = numbers(blocks[1]);

    ids.iter()
        .filter(|id| ranges.iter().any(|range| range.contains(id)))
        .count()
        .to_string()
}

pub fn part2(input: String) -> String {
    let blocks = blocks(&input);
    let mut ranges = blocks[0]
        .lines()
        .map(|line| {
            let mut parts = line.split('-');
            let start = parts.next().unwrap().parse::<i64>().unwrap();
            let end = parts.next().unwrap().parse::<i64>().unwrap();
            (start, end)
        })
        .sorted_by(|(start_a, _), (start_b, _)| start_a.cmp(start_b));

    let mut current_range = ranges.next().expect("At least one range");
    let mut count = 0;

    for range in ranges {
        if range.0 > current_range.1 + 1 {
            count += current_range.1 - current_range.0 + 1;
            current_range = range;
        } else if range.1 > current_range.1 {
            current_range.1 = range.1;
        }
    }

    count += current_range.1 - current_range.0 + 1;
    count.to_string()
}
