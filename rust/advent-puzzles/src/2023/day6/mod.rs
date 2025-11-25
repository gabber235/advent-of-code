use itertools::Itertools;
use std::u64;

use nom::{
    character::complete::{newline, space1, u64},
    multi::separated_list1,
    IResult, Parser,
};
use nom_supreme::{tag::complete::tag, ParserExt};

pub fn part1(input: String) -> String {
    let races = parse_races(&input).unwrap().1;

    races
        .iter()
        .map(calculate_new_records)
        .product::<u64>()
        .to_string()
}

pub fn part2(input: String) -> String {
    let input = input.replace(" ", "");
    let race = parse_race(&input).unwrap().1;

    calculate_new_records(&race).to_string()
}

fn calculate_new_records(race: &Race) -> u64 {
    let first = (1..race.max_time)
        .find_or_last(|time| is_new_record(race, *time))
        .unwrap_or(0);
    let last = (1..race.max_time)
        .rev()
        .find_or_last(|time| is_new_record(race, *time))
        .unwrap_or(0);

    (first..=last).count() as u64
}

fn is_new_record(race: &Race, hold_time: u64) -> bool {
    calculate_distance(hold_time, race.max_time) > race.record
}

fn calculate_distance(hold_time: u64, max_time: u64) -> u64 {
    (max_time - hold_time) * hold_time
}

#[derive(Debug, PartialEq, Eq)]
struct Race {
    max_time: u64,
    record: u64,
}

impl Race {
    fn new(max_time: u64, record: u64) -> Self {
        Self { max_time, record }
    }
}

fn parse_races(input: &str) -> IResult<&str, Vec<Race>> {
    let (input, times) = parse_times(input)?;
    let (input, _) = newline(input)?;
    let (input, distances) = parse_distances(input)?;

    let races = times
        .iter()
        .zip(distances.iter())
        .map(|(time, distance)| Race::new(*time, *distance))
        .collect();

    Ok((input, races))
}

fn parse_times(input: &str) -> IResult<&str, Vec<u64>> {
    tag("Time:")
        .precedes(space1)
        .precedes(separated_list1(space1, u64))
        .parse(input)
}

fn parse_distances(input: &str) -> IResult<&str, Vec<u64>> {
    tag("Distance:")
        .precedes(space1)
        .precedes(separated_list1(space1, u64))
        .parse(input)
}

fn parse_race(input: &str) -> IResult<&str, Race> {
    let (input, max_time) = tag("Time:").precedes(u64).parse(input)?;
    let (input, _) = newline(input)?;
    let (input, record) = tag("Distance:").precedes(u64).parse(input)?;

    Ok((input, Race::new(max_time, record)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_times() {
        let input = "Time:      7  15   30";
        let expected = vec![7, 15, 30];

        let (_, actual) = parse_times(input).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_parse_distances() {
        let input = "Distance:  9  40  200";
        let expected = vec![9, 40, 200];

        let (_, actual) = parse_distances(input).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_parse() {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        let expected = vec![Race::new(7, 9), Race::new(15, 40), Race::new(30, 200)];

        let (_, actual) = parse_races(input).unwrap();

        assert_eq!(expected, actual);
    }

    macro_rules! test_calculate_distance {
        ($name:ident, $hold_time:expr, $max_time:expr, $expected:expr) => {
            #[test]
            fn $name() {
                let actual = calculate_distance($hold_time, $max_time);

                assert_eq!($expected, actual);
            }
        };
    }

    test_calculate_distance!(test_calculate_distance_1, 0, 7, 0);
    test_calculate_distance!(test_calculate_distance_2, 1, 7, 6);
    test_calculate_distance!(test_calculate_distance_3, 2, 7, 10);
    test_calculate_distance!(test_calculate_distance_4, 3, 7, 12);
    test_calculate_distance!(test_calculate_distance_5, 4, 7, 12);
    test_calculate_distance!(test_calculate_distance_6, 5, 7, 10);
    test_calculate_distance!(test_calculate_distance_7, 6, 7, 6);
    test_calculate_distance!(test_calculate_distance_8, 7, 7, 0);
}
