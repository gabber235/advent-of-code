use std::i64;

use nom::{
    bytes::complete::{tag, take},
    character::complete::{digit1, newline},
    multi::separated_list1,
    IResult,
};

use crate::utils::{direction::Direction, point::Point};

pub fn part1(input: String) -> String {
    let instructions = parse_input(&input).unwrap().1;

    find_area(&instructions).to_string()
}

pub fn part2(input: String) -> String {
    let instructions = parse_input2(&input).unwrap().1;

    find_area(&instructions).to_string()
}

fn find_area(instructions: &[Instruction]) -> i64 {
    let mut point = Point::new(0, 0);
    let mut area: i64 = 0;
    let mut perimeter: i64 = 0;

    for instruction in instructions {
        let new_point =
            point.move_n_in_direction(instruction.direction, instruction.distance as i32 * 2);

        area += point.x as i64 * new_point.y as i64 - new_point.x as i64 * point.y as i64;
        perimeter += instruction.distance as i64 * 2;

        point = new_point;
    }

    // Final area calculation
    (area / 2 + perimeter) / 4 + 1
}

fn parse_input(input: &str) -> IResult<&str, Vec<Instruction>> {
    separated_list1(newline, parse_first_instruction)(input)
}

fn parse_first_instruction(input: &str) -> IResult<&str, Instruction> {
    let (input, direction) = parse_direction(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, distance) = digit1(input)?;
    let (input, _) = take(10usize)(input)?;

    Ok((
        input,
        Instruction {
            direction,
            distance: distance.parse().unwrap(),
        },
    ))
}

fn parse_direction(input: &str) -> IResult<&str, Direction> {
    let (input, direction) = take(1usize)(input)?;
    Ok((input, direction.parse().unwrap()))
}

fn parse_input2(input: &str) -> IResult<&str, Vec<Instruction>> {
    separated_list1(newline, parse_first_instruction2)(input)
}

fn parse_first_instruction2(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = take(2usize)(input)?;
    let (input, _) = digit1(input)?;
    let (input, _) = tag(" (#")(input)?;
    let (input, distance) = take(5usize)(input)?;
    let (input, direction) = take(1usize)(input)?;
    let (input, _) = tag(")")(input)?;

    let distance = u32::from_str_radix(distance, 16).unwrap();
    let direction = match direction {
        "0" => Direction::East,
        "1" => Direction::South,
        "2" => Direction::West,
        "3" => Direction::North,
        _ => panic!("Unknown direction"),
    };

    Ok((
        input,
        Instruction {
            direction,
            distance,
        },
    ))
}

#[derive(Debug)]
struct Instruction {
    direction: Direction,
    distance: u32,
}
