use std::fmt::Display;
use std::i32;
use std::vec;

use array2d::Array2D;

use crate::utils::map::GenerateMap;
use crate::utils::map::InteractWithPoint;
use crate::utils::map::PrintMap;
use crate::utils::point::Point;

pub fn part1(input: String) -> String {
    let mut grid = Array2D::generate_map(&input, |_, c| Spot::from(c)).unwrap();

    tilt_north(&mut grid);

    grid.print_map();

    calculate_north_load(&grid).to_string()
}

pub fn part2(input: String) -> String {
    let mut grid = Array2D::generate_map(&input, |_, c| Spot::from(c)).unwrap();

    let mut seen = vec![];
    let mut loop_start = 0;
    let mut loop_size = 0;

    for i in 0..10000 {
        tilt_north(&mut grid);
        tilt_west(&mut grid);
        tilt_south(&mut grid);
        tilt_east(&mut grid);

        if seen.contains(&grid) {
            loop_start = seen.iter().position(|g| *g == grid).unwrap();
            loop_size = seen.len() - loop_start;
            println!(
                "Loop found at i: {}, loop_start: {}, loop_size: {}",
                i, loop_start, loop_size
            );
            break;
        }

        seen.push(grid.clone());
    }

    let i = 1000000000 - 1;
    let index = (i - loop_start) % loop_size + loop_start;
    let grid = &seen[index];

    calculate_north_load(&grid).to_string()
}

fn tilt_north(grid: &mut Array2D<Spot>) {
    let mut swaps = vec![];

    for x in 0..grid.num_columns() {
        let mut roll_pos = 0;
        for y in 0..grid.num_rows() {
            let spot = *grid.get_point(&Point::new(x as i32, y as i32)).unwrap();
            match spot {
                Spot::Empty => {}
                Spot::FixedRock => {
                    roll_pos = y + 1;
                }
                Spot::RoundRock => {
                    swaps.push((
                        Point::new(x as i32, y as i32),
                        Point::new(x as i32, roll_pos as i32),
                    ));
                    roll_pos += 1;
                }
            }
        }
    }

    apply_swaps(grid, &swaps);
}

fn tilt_south(grid: &mut Array2D<Spot>) {
    let mut swaps = vec![];

    for x in 0..grid.num_columns() {
        let mut roll_pos: i32 = grid.num_rows() as i32 - 1;
        for y in (0..grid.num_rows()).rev() {
            let spot = *grid.get_point(&Point::new(x as i32, y as i32)).unwrap();
            match spot {
                Spot::Empty => {}
                Spot::FixedRock => {
                    roll_pos = y as i32 - 1;
                }
                Spot::RoundRock => {
                    swaps.push((
                        Point::new(x as i32, y as i32),
                        Point::new(x as i32, roll_pos as i32),
                    ));
                    roll_pos -= 1;
                }
            }
        }
    }

    apply_swaps(grid, &swaps);
}

fn tilt_east(grid: &mut Array2D<Spot>) {
    let mut swaps = vec![];

    for y in 0..grid.num_rows() {
        let mut roll_pos: i32 = grid.num_columns() as i32 - 1;
        for x in (0..grid.num_columns()).rev() {
            let spot = *grid.get_point(&Point::new(x as i32, y as i32)).unwrap();
            match spot {
                Spot::Empty => {}
                Spot::FixedRock => {
                    roll_pos = x as i32 - 1;
                }
                Spot::RoundRock => {
                    swaps.push((
                        Point::new(x as i32, y as i32),
                        Point::new(roll_pos as i32, y as i32),
                    ));
                    roll_pos -= 1;
                }
            }
        }
    }

    apply_swaps(grid, &swaps);
}

fn tilt_west(grid: &mut Array2D<Spot>) {
    let mut swaps = vec![];

    for y in 0..grid.num_rows() {
        let mut roll_pos = 0;
        for x in 0..grid.num_columns() {
            let spot = *grid.get_point(&Point::new(x as i32, y as i32)).unwrap();
            match spot {
                Spot::Empty => {}
                Spot::FixedRock => {
                    roll_pos = x + 1;
                }
                Spot::RoundRock => {
                    swaps.push((
                        Point::new(x as i32, y as i32),
                        Point::new(roll_pos as i32, y as i32),
                    ));
                    roll_pos += 1;
                }
            }
        }
    }

    apply_swaps(grid, &swaps);
}

fn apply_swaps(grid: &mut Array2D<Spot>, swaps: &[(Point, Point)]) {
    for (from, to) in swaps {
        let from_spot = *grid.get_point(from).unwrap();
        let to_spot = *grid.get_point(to).unwrap();
        grid.set_point(from, to_spot).unwrap();
        grid.set_point(to, from_spot).unwrap();
    }
}

fn calculate_north_load(grid: &Array2D<Spot>) -> usize {
    let lenn = grid.num_columns();
    grid.rows_iter()
        .enumerate()
        .map(|(y, row)| {
            let count = row
                .enumerate()
                .filter(|(_, spot)| **spot == Spot::RoundRock)
                .count();
            count * (lenn - y)
        })
        .sum()
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Spot {
    Empty,
    FixedRock,
    RoundRock,
}

impl From<char> for Spot {
    fn from(c: char) -> Self {
        match c {
            '.' => Spot::Empty,
            '#' => Spot::FixedRock,
            'O' => Spot::RoundRock,
            _ => panic!("Invalid spot: {}", c),
        }
    }
}

impl Display for Spot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Spot::Empty => '.',
            Spot::FixedRock => '#',
            Spot::RoundRock => 'O',
        };
        write!(f, "{}", c)
    }
}
