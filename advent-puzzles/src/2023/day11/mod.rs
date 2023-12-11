use std::fmt::Display;

use array2d::Array2D;
use itertools::Itertools;

use crate::utils::{
    map::{GenerateMap, IterAll},
    point::Point,
};

pub fn part1(input: String) -> String {
    let map = parse_input(&input);
    let galaxy_points = find_galaxy_points(&map, 1);

    galaxy_points
        .iter()
        .combinations(2)
        .map(|c| c[0].manhattan_distance(c[1]))
        .sum::<usize>()
        .to_string()
}

pub fn part2(input: String) -> String {
    let map = parse_input(&input);
    let galaxy_points = find_galaxy_points(&map, 1000000 - 1);

    galaxy_points
        .iter()
        .combinations(2)
        .map(|c| c[0].manhattan_distance(c[1]))
        .sum::<usize>()
        .to_string()
}

fn find_galaxy_points(map: &Array2D<Spot>, expansion: usize) -> Vec<Point> {
    let rows = map
        .rows_iter()
        .map(|row| row.collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let row_expansions = line_expansion(rows, expansion);

    let cols = map
        .columns_iter()
        .map(|col| col.collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let col_expansions = line_expansion(cols, expansion);

    map.iter_all()
        .filter(|(_, s)| **s == Spot::Galaxy)
        .map(|(p, _)| p)
        .map(|p| extrapolate_point(p, &row_expansions, &col_expansions))
        .collect()
}

fn line_expansion(lines: Vec<Vec<&Spot>>, expansion: usize) -> Vec<usize> {
    lines
        .into_iter()
        .map(|line| match line.iter().all(|s| **s == Spot::Empty) {
            true => expansion,
            false => 0,
        })
        .scan(0, |state, x| {
            *state += x;
            Some(*state)
        })
        .collect::<Vec<_>>()
}

fn extrapolate_point(point: Point, row_expansions: &[usize], col_expansions: &[usize]) -> Point {
    let x = point.x + col_expansions[point.x as usize] as i32;
    let y = point.y + row_expansions[point.y as usize] as i32;

    Point::new(x, y)
}

fn parse_input(input: &str) -> Array2D<Spot> {
    Array2D::generate_map(input, |_, c| Spot::from(c)).unwrap()
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Spot {
    Empty,
    Galaxy,
}

impl From<char> for Spot {
    fn from(c: char) -> Self {
        match c {
            '.' => Self::Empty,
            '#' => Self::Galaxy,
            _ => panic!("Invalid spot"),
        }
    }
}

impl Display for Spot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Spot::Empty => write!(f, "."),
            Spot::Galaxy => write!(f, "#"),
        }
    }
}
