use std::{collections::HashMap, fmt::Display, io::stdin};

use array2d::Array2D;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::utils::{
    direction::{self, Direction},
    map::{GenerateMap, InteractWithPoint, IterAll, PrintMap},
    point::Point,
};

pub fn part1(input: String) -> String {
    let grid: Array2D<Tile> = Array2D::generate_map(&input, |_, c| c.into()).unwrap();

    let start_beam = Beam::new(Direction::East, Point::new(0, 0));
    find_enerized_configuraton(&grid, start_beam).to_string()
}

pub fn part2(input: String) -> String {
    // Find the highest energy configuration for all starting beams
    let grid: Array2D<Tile> = Array2D::generate_map(&input, |_, c| c.into()).unwrap();

    let mut starting_beams = Vec::new();

    // Top row
    for x in 0..grid.num_columns() {
        starting_beams.push(Beam::new(Direction::South, Point::new(x as i32, 0)));
    }

    // Right column
    for y in 0..grid.num_rows() {
        starting_beams.push(Beam::new(
            Direction::West,
            Point::new(grid.num_columns() as i32 - 1, y as i32),
        ));
    }

    // Bottom row
    for x in 0..grid.num_columns() {
        starting_beams.push(Beam::new(
            Direction::North,
            Point::new(x as i32, grid.num_rows() as i32 - 1),
        ));
    }

    // Left column
    for y in 0..grid.num_rows() {
        starting_beams.push(Beam::new(Direction::East, Point::new(0, y as i32)));
    }

    starting_beams
        .into_par_iter()
        .map(|start_beam| find_enerized_configuraton(&grid, start_beam))
        .max()
        .unwrap()
        .to_string()
}

fn find_enerized_configuraton(grid: &Array2D<Tile>, start_beam: Beam) -> usize {
    let mut beams = vec![start_beam];

    let mut energies: HashMap<Point, u8> = HashMap::new();

    while !beams.is_empty() {
        for beam in beams.iter() {
            let energy = energies.get(&beam.point).unwrap_or(&0);
            energies.insert(beam.point, set_beam_in_direction(*energy, &beam.direction));
        }

        beams = beams
            .into_iter()
            .flat_map(|beam| step_beam(&grid, beam))
            .filter(|beam| beam.point.ok_map(&grid).is_some())
            .filter(|beam| {
                energies
                    .get(&beam.point)
                    .map(|energy| !has_beam_in_direction(*energy, &beam.direction))
                    .unwrap_or(true)
            })
            .collect();
    }

    energies.len()
}

fn has_beam_in_direction(configuration: u8, direction: &Direction) -> bool {
    // the last 4 bits represent the directions. 1 means there is a beam in that direction.
    // 0b0000_0001 = North
    // 0b0000_0010 = East
    // 0b0000_0100 = South
    // 0b0000_1000 = West
    let mask = match direction {
        Direction::North => 0b0000_0001,
        Direction::East => 0b0000_0010,
        Direction::South => 0b0000_0100,
        Direction::West => 0b0000_1000,
    };

    configuration & mask != 0
}

fn set_beam_in_direction(configuration: u8, direction: &Direction) -> u8 {
    // the last 4 bits represent the directions. 1 means there is a beam in that direction.
    // 0b0000_0001 = North
    // 0b0000_0010 = East
    // 0b0000_0100 = South
    // 0b0000_1000 = West
    let mask = match direction {
        Direction::North => 0b0000_0001,
        Direction::East => 0b0000_0010,
        Direction::South => 0b0000_0100,
        Direction::West => 0b0000_1000,
    };

    configuration | mask
}

fn step_beam(grid: &Array2D<Tile>, beam: Beam) -> Vec<Beam> {
    let tile = grid.get_point(&beam.point).unwrap();

    match tile {
        Tile::Empty => {
            vec![Beam::new_step(beam.direction, beam.point)]
        }
        Tile::HorizontalSplit => match beam.direction {
            Direction::North | Direction::South => {
                vec![
                    Beam::new_step(Direction::East, beam.point),
                    Beam::new_step(Direction::West, beam.point),
                ]
            }
            _ => {
                vec![Beam::new_step(beam.direction, beam.point)]
            }
        },
        Tile::VerticalSplit => match beam.direction {
            Direction::East | Direction::West => {
                vec![
                    Beam::new_step(Direction::North, beam.point),
                    Beam::new_step(Direction::South, beam.point),
                ]
            }
            _ => {
                vec![Beam::new_step(beam.direction, beam.point)]
            }
        },
        Tile::ForwardMirror => match beam.direction {
            Direction::North => {
                vec![Beam::new_step(Direction::East, beam.point)]
            }
            Direction::East => {
                vec![Beam::new_step(Direction::North, beam.point)]
            }
            Direction::South => {
                vec![Beam::new_step(Direction::West, beam.point)]
            }
            Direction::West => {
                vec![Beam::new_step(Direction::South, beam.point)]
            }
        },
        Tile::BackwardMirror => match beam.direction {
            Direction::North => {
                vec![Beam::new_step(Direction::West, beam.point)]
            }
            Direction::East => {
                vec![Beam::new_step(Direction::South, beam.point)]
            }
            Direction::South => {
                vec![Beam::new_step(Direction::East, beam.point)]
            }
            Direction::West => {
                vec![Beam::new_step(Direction::North, beam.point)]
            }
        },
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Beam {
    direction: Direction,
    point: Point,
}

impl Beam {
    fn new(direction: Direction, point: Point) -> Self {
        Self { direction, point }
    }

    fn new_step(direction: Direction, point: Point) -> Self {
        Self {
            direction,
            point: point.move_in_direction(direction),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    VerticalSplit,
    HorizontalSplit,
    ForwardMirror,
    BackwardMirror,
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '.' => Self::Empty,
            '|' => Self::VerticalSplit,
            '-' => Self::HorizontalSplit,
            '/' => Self::ForwardMirror,
            '\\' => Self::BackwardMirror,
            _ => panic!("Invalid tile: {}", c),
        }
    }
}
