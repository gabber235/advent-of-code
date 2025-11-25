use std::ops::{self};
use std::u64;

use itertools::Itertools;
use nom::combinator::value;
use nom::{
    branch::alt, bytes::complete::tag, character::complete::digit1, multi::separated_list1, IResult,
};
use strum_macros::Display;

pub fn part1(input: String) -> String {
    let (group, mappings) = parse(&input);

    let mut group = group;

    while group.type_ != Type::Location {
        // println!("{:?}", group);
        group = transform(group, &mappings);
    }

    let min = group.values.iter().min().unwrap();

    format!("{}", min)
}

fn transform(group: Group, mappings: &[Mapping]) -> Group {
    let mapper = find_mapper(group.type_.clone(), mappings);
    let Some(mapper) = mapper else {
        return group;
    };

    transform_mapping(group, mapper)
}

fn transform_mapping(group: Group, mapping: &Mapping) -> Group {
    let mut old_values = group.values.clone();
    let mut values = vec![];

    for range in &mapping.ranges {
        let matching_values = old_values
            .iter()
            .filter(|value| range.contains(**value))
            .collect::<Vec<_>>();

        values.extend(
            matching_values
                .iter()
                .map(|value| range.map(**value).unwrap()),
        );

        old_values.retain(|value| !range.contains(*value));
    }

    values.extend(old_values);

    Group {
        type_: mapping.to.clone(),
        values,
    }
}

pub fn part2(input: String) -> String {
    let (group, mappings) = parse(&input);

    // The group was incorrect.
    let ranges = group
        .values
        .iter()
        .tuples()
        .map(|(start, length)| ops::Range {
            start: *start,
            end: *start + *length,
        })
        .collect::<Vec<_>>();

    let mut group = GroupRange {
        type_: group.type_,
        ranges,
    };

    while group.type_ != Type::Location {
        if let Some(mapper) = find_mapper(group.type_.clone(), &mappings) {
            group = transform_range_mapping(group, mapper);
        } else {
            break;
        }
    }

    let min = group
        .ranges
        .iter()
        .map(|range| range.start)
        .min()
        .unwrap_or(u64::MAX);

    format!("{}", min)
}

fn transform_range_mapping(group: GroupRange, mapping: &Mapping) -> GroupRange {
    let ranges = group
        .ranges
        .iter()
        .flat_map(|range| transform_range(range, mapping).into_iter())
        .collect::<Vec<_>>();

    GroupRange {
        type_: mapping.to.clone(),
        ranges,
    }
}

fn transform_range(range: &ops::Range<u64>, mapping: &Mapping) -> Vec<ops::Range<u64>> {
    let intersections = mapping
        .ranges
        .iter()
        .filter_map(|mapping_range| mapping_range.intersection(&range))
        .collect::<Vec<_>>();

    // All the ranges withing the range that are not mapped should be kept.
    let mut ranges = vec![];

    let mut start = range.start;
    let flattened = flatten_ranges(
        intersections
            .iter()
            .map(|(a, _)| a.clone())
            .collect::<Vec<_>>(),
    );
    for intersection in flattened {
        if start < intersection.start {
            ranges.push(ops::Range {
                start,
                end: intersection.start,
            });
        }
        start = intersection.end;
    }

    if start < range.end {
        ranges.push(ops::Range {
            start,
            end: range.end,
        });
    }

    ranges.extend(
        intersections
            .iter()
            .map(|(_, b)| b.clone())
            .collect::<Vec<_>>(),
    );

    ranges
}

fn flatten_ranges(ranges: Vec<ops::Range<u64>>) -> Vec<ops::Range<u64>> {
    let ranges = ranges
        .iter()
        .sorted_by(|a, b| a.start.cmp(&b.start))
        .collect::<Vec<_>>();

    let mut flattened: Vec<ops::Range<u64>> = vec![];

    for range in ranges {
        let current = match flattened.last_mut() {
            Some(last) => {
                if last.overlaps(range) {
                    last.merge_range(range);
                    continue;
                }
                range.clone()
            }
            None => range.clone(),
        };
        flattened.push(current);
    }

    flattened
}

fn find_mapper(type_: Type, mappings: &[Mapping]) -> Option<&Mapping> {
    mappings.iter().find(|mapping| mapping.from == type_)
}

fn parse(input: &str) -> (Group, Vec<Mapping>) {
    let blocks = input.split("\n\n").collect::<Vec<_>>();

    let group = parse_group(blocks[0]).unwrap().1;

    let mappings = blocks[1..]
        .iter()
        .map(|block| parse_mapping(block).unwrap().1)
        .collect::<Vec<_>>();

    (group, mappings)
}

fn parse_group(input: &str) -> IResult<&str, Group> {
    let (input, _) = tag("seeds: ")(input)?;
    let (input, values) = separated_list1(tag(" "), digit1)(input)?;

    let values = values
        .iter()
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    Ok((
        input,
        Group {
            type_: Type::Seed,
            values,
        },
    ))
}

fn parse_mapping(input: &str) -> IResult<&str, Mapping> {
    let lines = input.lines().collect::<Vec<_>>();

    let (input, (from, to)) = parse_source_destination(lines[0]).unwrap();

    let ranges = lines[1..]
        .iter()
        .map(|line| parse_range(line).unwrap().1)
        .sorted()
        .collect::<Vec<_>>();

    Ok((input, Mapping { from, to, ranges }))
}

fn parse_source_destination(input: &str) -> IResult<&str, (Type, Type)> {
    let (input, from) = parse_type(input)?;
    let (input, _) = tag("-to-")(input)?;
    let (input, to) = parse_type(input)?;

    Ok((input, (from, to)))
}

fn parse_range(input: &str) -> IResult<&str, Range> {
    let (input, numbers) = separated_list1(tag(" "), digit1)(input)?;

    let numbers = numbers
        .iter()
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let (destination, source, length) = match numbers.len() {
        3 => (numbers[0], numbers[1], numbers[2]),
        _ => panic!("Invalid range: {:?}", numbers),
    };

    Ok((
        input,
        Range {
            source,
            destination,
            length,
        },
    ))
}

fn parse_type(input: &str) -> IResult<&str, Type> {
    alt((
        value(Type::Seed, tag("seed")),
        value(Type::Soil, tag("soil")),
        value(Type::Fertilizer, tag("fertilizer")),
        value(Type::Water, tag("water")),
        value(Type::Light, tag("light")),
        value(Type::Temperature, tag("temperature")),
        value(Type::Humidity, tag("humidity")),
        value(Type::Location, tag("location")),
    ))(input)
}

#[derive(Debug, Clone)]
struct Group {
    type_: Type,
    values: Vec<u64>,
}

#[derive(Debug, Clone)]
struct GroupRange {
    type_: Type,
    ranges: Vec<ops::Range<u64>>,
}

#[derive(Debug, Clone)]
struct Mapping {
    from: Type,
    to: Type,
    ranges: Vec<Range>,
}

#[derive(Debug, Clone, PartialEq, Eq, Ord)]
struct Range {
    source: u64,
    destination: u64,
    length: u64,
}

impl Range {
    fn contains(&self, value: u64) -> bool {
        value >= self.source && value <= self.source + self.length
    }

    fn map(&self, value: u64) -> Option<u64> {
        if !self.contains(value) {
            return None;
        }
        let offset = value - self.source;
        Some(self.destination + offset)
    }

    fn intersection(&self, range: &ops::Range<u64>) -> Option<(ops::Range<u64>, ops::Range<u64>)> {
        let start = std::cmp::max(self.source, range.start);
        let end = std::cmp::min(self.source + self.length, range.end);

        if start > end {
            return None;
        }

        let source = (start - self.source) + self.destination;
        let destination = (end - self.source) + self.destination;

        Some((
            ops::Range { start, end },
            ops::Range {
                start: source,
                end: destination,
            },
        ))
    }
}

impl PartialOrd for Range {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.source.partial_cmp(&other.source)
    }
}

#[derive(Debug, Clone, Display, PartialEq, Eq)]
enum Type {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}

trait RangeExt {
    fn overlaps(&self, other: &ops::Range<u64>) -> bool;
    fn merge_range(&mut self, other: &ops::Range<u64>);
}

impl RangeExt for ops::Range<u64> {
    fn overlaps(&self, other: &ops::Range<u64>) -> bool {
        self.start <= other.end && self.end >= other.start
            || other.start <= self.end && other.end >= self.start
    }

    fn merge_range(&mut self, other: &ops::Range<u64>) {
        self.start = std::cmp::min(self.start, other.start);
        self.end = std::cmp::max(self.end, other.end);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flatten_range() {
        let ranges = vec![
            ops::Range { start: 0, end: 10 },
            ops::Range { start: 3, end: 7 },
            ops::Range { start: 7, end: 12 },
            ops::Range { start: 15, end: 20 },
            ops::Range { start: 11, end: 16 },
        ];

        let flattened = flatten_ranges(ranges);

        assert_eq!(flattened, vec![ops::Range { start: 0, end: 20 },]);
    }
}
