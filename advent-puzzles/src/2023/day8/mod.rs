use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    bytes::complete::take,
    character::complete::newline,
    combinator::value,
    multi::{many1, separated_list1},
    IResult,
};

pub fn part1(input: String) -> String {
    let map = parse_map(&input).unwrap().1;

    let location = &Location("AAA".to_string());
    find_path_for_single_location(&map, &location).to_string()
}

pub fn part2(input: String) -> String {
    let map = parse_map(&input).unwrap().1;

    let locations = map
        .paths
        .keys()
        .into_iter()
        .filter(|l| l.0.ends_with("A"))
        .collect::<Vec<_>>();

    locations
        .iter()
        .map(|l| find_path_for_single_location(&map, l))
        .reduce(|a, b| num::integer::lcm(a, b))
        .unwrap()
        .to_string()
}

fn find_path_for_single_location(map: &Map, start_location: &Location) -> usize {
    let mut location = start_location;

    let mut steps = 0;

    while location.0.ends_with("Z") == false {
        let direction = map.directions.get(steps % map.directions.len()).unwrap();
        steps += 1;
        let (left, right) = map.paths.get(&location).unwrap();

        location = match direction {
            Direction::Left => left,
            Direction::Right => right,
        };
    }

    steps
}

#[derive(Debug, Clone)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Location(String);

#[derive(Debug)]
struct Map {
    directions: Vec<Direction>,
    paths: HashMap<Location, (Location, Location)>,
}

fn parse_map(input: &str) -> IResult<&str, Map> {
    let (input, directions) = many1(alt((
        value(Direction::Left, tag("L")),
        value(Direction::Right, tag("R")),
    )))(input)?;

    let (input, _) = many1(newline)(input)?;
    let (input, paths) = separated_list1(newline, parse_path)(input)?;

    let paths: HashMap<_, _> = paths
        .into_iter()
        .map(|(origin, left, right)| (origin, (left, right)))
        .collect();

    Ok((input, Map { directions, paths }))
}

fn parse_path(input: &str) -> IResult<&str, (Location, Location, Location)> {
    let (input, origin) = parse_location(input)?;
    let (input, _) = tag(" = (")(input)?;
    let (input, left) = parse_location(input)?;
    let (input, _) = tag(", ")(input)?;
    let (input, right) = parse_location(input)?;
    let (input, _) = tag(")")(input)?;

    Ok((input, (origin, left, right)))
}

fn parse_location(input: &str) -> IResult<&str, Location> {
    let (input, location) = take(3usize)(input)?;
    Ok((input, Location(location.to_string())))
}
