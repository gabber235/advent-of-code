use std::fmt::Display;

use array2d::Array2D;

use crate::utils::{
    direction::Direction,
    map::{GenerateMap, GetWithPoint, IterAll},
    point::Point,
};

pub fn part1(input: String) -> String {
    let map = parse_map(&input);

    let numbers = parse_numbers(&map);

    let sum = numbers
        .iter()
        .filter(|n| is_connected(&map, &n))
        .map(|n| n.value)
        .sum::<u32>();

    format!("{sum}")
}

pub fn part2(input: String) -> String {
    let map = parse_map(&input);

    let numbers = parse_numbers(&map);

    let ratios = map
        .iter_all()
        .filter(|(_, c)| **c == Spot::Symbol('*'))
        .flat_map(|(p, _)| find_connecting_gears(&numbers, &p))
        .map(|(n1, n2)| n1.value * n2.value)
        .sum::<u32>();

    format!("{ratios}")
}

fn parse_map(input: &str) -> Array2D<Spot> {
    Array2D::generate_map(input, |_, c| match c {
        '.' => Spot::Empty,
        c if c.is_digit(10) => Spot::Digit(c.to_digit(10).unwrap()),
        c => Spot::Symbol(c),
    })
    .unwrap()
}

fn is_connected(map: &Array2D<Spot>, number: &Number) -> bool {
    for x in -1..=(number.width as i32) {
        for y in -1..=1 {
            let point = Point::new(number.position.x + x, number.position.y + y);
            if let Some(Spot::Symbol(_)) = map.get_point(&point) {
                return true;
            }
        }
    }
    false
}

fn find_connecting_gears(numbers: &[Number], point: &Point) -> Option<(Number, Number)> {
    let neighbors = point.neighbours_all_directions();

    let connecting_numbers = numbers
        .iter()
        .filter(|n| neighbors.iter().any(|p| n.is_inside(p)))
        .collect::<Vec<_>>();

    if connecting_numbers.len() != 2 {
        return None;
    }

    let n1 = connecting_numbers[0];
    let n2 = connecting_numbers[1];

    return Some((n1.clone(), n2.clone()));
}

fn parse_numbers(map: &Array2D<Spot>) -> Vec<Number> {
    let digits = map.iter_all().filter_map(|(p, s)| match s {
        Spot::Digit(d) => Some((p, *d)),
        _ => None,
    });

    let mut numbers = Vec::new();
    let mut current: Option<Number> = None;

    for (point, digit) in digits {
        if let Some(mut n) = current {
            match n
                .position
                .move_n_in_direction(Direction::East, n.width as i32)
                .ok_map(map)
            {
                Some(p) if p == point => {
                    n.width += 1;
                    n.value = n.value * 10 + digit;
                    current = Some(n);
                    continue;
                }
                _ => {
                    numbers.push(n);
                }
            }
        }

        current = Some(Number {
            value: digit,
            position: point,
            width: 1,
        });
    }

    if let Some(n) = current {
        numbers.push(n);
    }

    numbers
}

#[derive(Debug, Clone)]
struct Number {
    value: u32,
    position: Point,
    width: usize,
}

impl Number {
    fn is_inside(&self, point: &Point) -> bool {
        point.x >= self.position.x
            && point.x < self.position.x + self.width as i32
            && point.y == self.position.y
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum Spot {
    Empty,
    Digit(u32),
    Symbol(char),
}

impl Display for Spot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Spot::Empty => write!(f, "."),
            Spot::Digit(d) => write!(f, "{}", d),
            Spot::Symbol(s) => write!(f, "{}", s),
        }
    }
}
