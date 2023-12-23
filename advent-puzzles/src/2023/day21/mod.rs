use std::{collections::HashMap, fmt::Display, hash::Hasher};

use array2d::Array2D;
use itertools::Itertools;
use pathfinding::directed::astar::astar;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::utils::{
    map::{GenerateMap, InteractWithPoint},
    point::Point,
};

pub fn part1(input: String) -> String {
    let mut start = Point::default();
    let map = Array2D::generate_map(&input, |p, c| {
        if c == 'S' {
            start = p;
            Tile::GardenPlot
        } else {
            Tile::from(c)
        }
    })
    .expect("Failed to generate map");

    let steps = 64;

    calculate_plots(&map, steps, &start).to_string()
}

pub fn part2(input: String) -> String {
    let mut start = Point::default();
    let map = Array2D::generate_map(&input, |p, c| {
        if c == 'S' {
            start = p;
            Tile::GardenPlot
        } else {
            Tile::from(c)
        }
    })
    .expect("Failed to generate map");

    let height = map.num_rows() as i32;
    let target = 26501365;

    let mod_target = target % height;

    let calculations = (0..=2)
        .into_par_iter()
        .map(|times| calculate_plots(&map, mod_target + height * times, &start))
        .collect::<Vec<_>>();

    let base = calculations[0];
    let base_with_height = calculations[1];
    let base_with_double_height = calculations[2];

    let first_diff1 = base_with_height - base;
    let first_diff2 = base_with_double_height - base_with_height;
    let second_diff = first_diff2 - first_diff1;

    // https://www.radfordmathematics.com/algebra/sequences-series/difference-method-sequences/quadratic-sequences.html
    let a = second_diff / 2;
    let b = first_diff1 - 3 * a;
    let c = base - a - b;

    let calculate = |n: i64| a * n * n + b * n + c;

    let times = (target as i64 - mod_target as i64) / height as i64;

    calculate(times + 1).to_string()
}

fn calculate_plots(map: &Array2D<Tile>, steps: i32, start_point: &Point) -> i64 {
    let mut cache = HashMap::new();
    cache.insert(start_point.clone(), 0);

    (-steps..=steps)
        .flat_map(|y| {
            let diff = steps - y.abs();
            (-diff..=diff)
                .step_by(2)
                .map(move |x| Point::new(start_point.x + x, start_point.y + y))
        })
        .sorted_by(|a, b| {
            a.manhattan_distance(start_point)
                .cmp(&b.manhattan_distance(start_point))
        })
        .filter(|p| validate_point(map, p, steps, start_point, &mut cache))
        .count() as i64
}

fn validate_point(
    map: &Array2D<Tile>,
    point: &Point,
    steps: i32,
    start_point: &Point,
    cache: &mut HashMap<Point, i32>,
) -> bool {
    if *map.get_looping_point(point) != Tile::GardenPlot {
        return false;
    }

    let Some((path, cost)) = astar(
        &PathPoint::new(*point, 0),
        |pp| {
            pp.point
                .neighbours()
                .iter()
                .filter(|neighbour| {
                    let y = neighbour.y - start_point.y;
                    let diff = steps - y.abs();
                    let x = neighbour.x - start_point.x;
                    x.abs() <= diff
                })
                .filter(|p| *map.get_looping_point(p) == Tile::GardenPlot)
                .map(|p| (PathPoint::new(*p, pp.cost + 1), 1))
                .collect::<Vec<_>>()
        },
        |pp| pp.point.manhattan_distance(start_point),
        |pp| {
            if pp.point == *start_point {
                return true;
            }

            if let Some(c) = cache.get(&pp.point) {
                return *c + pp.cost <= steps;
            }
            false
        },
    ) else {
        return false;
    };

    let last = path.last().unwrap().point;
    let last_cost = cache.get(&last).unwrap();
    let true_cost = last_cost + cost as i32;

    cache.insert(point.clone(), true_cost);

    true_cost <= steps
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    GardenPlot,
    Rock,
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '#' => Tile::Rock,
            '.' => Tile::GardenPlot,
            _ => panic!("Invalid tile"),
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Tile::GardenPlot => '.',
            Tile::Rock => '#',
        };
        write!(f, "{}", c)
    }
}

#[derive(Debug, Clone, Copy, Eq)]
struct PathPoint {
    point: Point,
    cost: i32,
}

impl PathPoint {
    fn new(point: Point, cost: i32) -> Self {
        Self { point, cost }
    }
}

impl PartialEq for PathPoint {
    fn eq(&self, other: &Self) -> bool {
        self.point == other.point
    }
}

impl std::hash::Hash for PathPoint {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.point.hash(state);
    }
}
