use std::fmt::{Display, Formatter};

use num::Integer;

#[derive(Debug, PartialEq)]
enum Rotation {
    Left(i16),
    Right(i16),
}

impl Display for Rotation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Rotation::Left(angle) => write!(f, "L{}", angle),
            Rotation::Right(angle) => write!(f, "R{}", angle),
        }
    }
}

pub fn part1(input: String) -> String {
    let rotations = input.lines().map(parse_rotation).collect::<Vec<_>>();

    let mut dial = 50;
    let mut reached_zero = 0;

    for rotation in rotations.iter() {
        match rotation {
            Rotation::Left(angle) => dial = (((dial - angle) % 100) + 100) % 100,
            Rotation::Right(angle) => dial = (((dial + angle) % 100) + 100) % 100,
        }
        if dial == 0 {
            reached_zero += 1;
        }
    }
    reached_zero.to_string()
}

pub fn part2(input: String) -> String {
    let rotations = input.lines().map(parse_rotation).collect::<Vec<_>>();

    let mut dial = 50;
    let mut reached_zero = 0;

    for rotation in rotations.iter() {
        match rotation {
            Rotation::Left(angle) => {
                let full_rotations = angle.div_floor(&100);
                let remainder = angle % 100;
                let crosses_zero = if dial > 0 && remainder >= dial { 1 } else { 0 };
                reached_zero += full_rotations + crosses_zero;
                dial = (((dial - remainder) % 100) + 100) % 100;
            }
            Rotation::Right(angle) => {
                let full_rotations = angle.div_floor(&100);
                let remainder = angle % 100;
                let crosses_zero = if dial + remainder >= 100 { 1 } else { 0 };
                reached_zero += full_rotations + crosses_zero;
                dial = (((dial + remainder) % 100) + 100) % 100;
            }
        }
    }
    reached_zero.to_string()
}

fn parse_rotation(input: &str) -> Rotation {
    let mut chars = input.chars();
    let direction = match chars.next() {
        Some('L') => Rotation::Left,
        Some('R') => Rotation::Right,
        _ => panic!("Invalid rotation"),
    };
    let angle = chars.as_str().parse().unwrap();
    direction(angle)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_rotation() {
        assert_eq!(parse_rotation("L90"), Rotation::Left(90));
        assert_eq!(parse_rotation("R20"), Rotation::Right(20));
    }
}
