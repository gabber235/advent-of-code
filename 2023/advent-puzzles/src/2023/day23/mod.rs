use std::fmt::Display;
use std::fmt::Write;

use array2d::Array2D;
use colored::Color;
use colored::Colorize;
use strum::IntoEnumIterator;

use crate::utils::{
    direction::Direction,
    map::GenerateMap,
    map::{InteractWithPoint, PrintMapWith},
    point::Point,
};

pub fn part1(input: String) -> String {
    let grid: Array2D<Tile> = Array2D::generate_map(&input, |_, c| c.into()).unwrap();

    let start_point = Point::new(1, 0);
    let end_point = Point::new(grid.num_rows() as i32 - 2, grid.num_columns() as i32 - 1);

    let current_path = vec![start_point];

    let longest_path =
        find_longest_possible_path_with_slopes(&grid, current_path, &end_point).unwrap();

    (longest_path.len() - 1).to_string()
}

fn find_longest_possible_path_with_slopes(
    grid: &Array2D<Tile>,
    mut current_path: Vec<Point>,
    end_point: &Point,
) -> Option<Vec<Point>> {
    loop {
        let current_point = current_path.last().unwrap();

        if *current_point == *end_point {
            return Some(current_path);
        }

        let possible_points = Direction::iter()
            .map(|direction| (direction, current_point + direction))
            .filter(|(direction, new_point)| {
                grid.get_point(new_point)
                    .map(|tile| tile.can_move_in_direction(*direction, true))
                    .unwrap_or(false)
            })
            .filter(|(_, new_point)| !current_path.contains(new_point))
            .map(|(_, new_point)| new_point)
            .collect::<Vec<_>>();

        if possible_points.is_empty() {
            return None;
        }

        if possible_points.len() == 1 {
            current_path.push(possible_points[0]);
            continue;
        }

        // print_grid(grid, &current_path);

        let mut longest_path: Option<Vec<Point>> = None;

        for new_point in possible_points {
            current_path.push(new_point);
            let path =
                find_longest_possible_path_with_slopes(grid, current_path.clone(), end_point);
            current_path.pop();

            if let Some(path) = path {
                if longest_path.is_none() || path.len() > longest_path.clone().unwrap().len() {
                    longest_path = Some(path.clone());
                }
            }
        }

        return longest_path;
    }
}
pub fn part2(input: String) -> String {
    let grid: Array2D<Tile> = Array2D::generate_map(&input, |_, c| c.into()).unwrap();

    let start_point = Point::new(1, 0);
    let end_point = Point::new(grid.num_rows() as i32 - 2, grid.num_columns() as i32 - 1);

    let segments = find_segments(&grid, &start_point, &end_point);

    let longest_path = find_longest_possible_path_with_segments(
        &grid,
        &segments,
        vec![],
        &[],
        &start_point,
        &end_point,
    )
    .expect("No path found");

    // print_grid(&grid, &segments, &longest_path, &[]);
    // println!();
    //
    // println!(
    //     "Paths sum: {}",
    //     longest_path
    //         .iter()
    //         .map(|segment| segment.length)
    //         .sum::<usize>()
    // );
    // println!("Segments: {}", longest_path.len());

    (longest_path
        .iter()
        .map(|segment| segment.length)
        .sum::<usize>()
        + longest_path.len())
    .to_string()
}

fn find_segments(grid: &Array2D<Tile>, start_point: &Point, end_point: &Point) -> Vec<Segment> {
    let mut segments: Vec<Segment> = Vec::new();

    let mut cross_points = vec![*start_point];

    while let Some(cross_point) = cross_points.pop() {
        for direction in Direction::iter() {
            let new_point = cross_point + direction;
            if grid
                .get_point(&new_point)
                .map(|tile| !tile.can_move_in_direction(direction, false))
                .unwrap_or(true)
            {
                continue;
            }

            if segments
                .iter()
                .any(|segment| segment.is_segment(&new_point))
            {
                continue;
            }

            if let Some(segment) = find_segment(grid, &cross_point, &end_point, direction) {
                segments.push(segment.clone());
                if !cross_points.contains(&segment.end_cross_point) {
                    cross_points.push(segment.end_cross_point);
                }
            }
        }
    }

    segments
}

fn find_segment(
    grid: &Array2D<Tile>,
    start_point: &Point,
    end_point: &Point,
    direction: Direction,
) -> Option<Segment> {
    let mut current_point = *start_point;
    let mut current_direction = direction;
    let mut current_length = 0;

    loop {
        let new = current_point + current_direction;

        if grid.get_point(&new).is_none() {
            return None;
        }

        let new_neighbours = Direction::iter()
            .filter(|direction| *direction != current_direction.opposite())
            .map(|direction| (direction, new + direction))
            .filter(|(direction, new_point)| {
                grid.get_point(new_point)
                    .map(|tile| tile.can_move_in_direction(*direction, false))
                    .unwrap_or(false)
            })
            .collect::<Vec<_>>();

        if new_neighbours.len() == 1 {
            current_point = new;
            current_direction = new_neighbours[0].0;
            current_length += 1;
            continue;
        }

        if new_neighbours.len() == 0 && new != *end_point {
            return None;
        }

        return Some(Segment {
            start_point: *start_point + direction,
            start_cross_point: *start_point,
            end_point: current_point,
            end_cross_point: new,
            length: current_length,
        });
    }
}

fn find_longest_possible_path_with_segments(
    grid: &Array2D<Tile>,
    segments: &[Segment],
    mut current_path: Vec<Segment>,
    blacklisted_segments: &[Segment],
    current_cross_point: &Point,
    end_point: &Point,
) -> Option<Vec<Segment>> {
    if *current_cross_point == *end_point {
        return Some(current_path);
    }

    let possible_segments = segments
        .iter()
        .filter(|segment| segment.is_cross_point(current_cross_point))
        .filter(|segment| !blacklisted_segments.contains(segment))
        .collect::<Vec<_>>();

    if possible_segments.is_empty() {
        return None;
    }

    let mut longest_path: Option<Vec<Segment>> = None;

    let mut blacklisted_segments = blacklisted_segments.to_vec();
    blacklisted_segments.extend(possible_segments.iter().copied());

    for segment in possible_segments {
        current_path.push(*segment);

        // print_grid(grid, &segments, &current_path, &blacklisted_segments);
        // stdin().read_line(&mut String::new()).unwrap();

        let new_cross_point = if segment.start_cross_point == *current_cross_point {
            segment.end_cross_point
        } else {
            segment.start_cross_point
        };

        let path = find_longest_possible_path_with_segments(
            grid,
            segments,
            current_path.clone(),
            &blacklisted_segments,
            &new_cross_point,
            end_point,
        );

        current_path.pop();

        if let Some(path) = path {
            if longest_path.is_none()
                || path.iter().map(|segment| segment.length).sum::<usize>()
                    > longest_path
                        .clone()
                        .unwrap()
                        .iter()
                        .map(|segment| segment.length)
                        .sum::<usize>()
            {
                longest_path = Some(path.clone());
            }
        }
    }

    longest_path
}

#[allow(dead_code)]
fn print_grid(
    grid: &Array2D<Tile>,
    segments: &[Segment],
    path: &[Segment],
    blacklisted_segments: &[Segment],
) {
    let possible_strings = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
    let possible_colors = vec![
        Color::Red,
        Color::Green,
        Color::Yellow,
        Color::Blue,
        Color::Magenta,
        Color::Cyan,
    ];
    let segments_with_points = segments
        .iter()
        .map(|segment| (segment, segment.points(grid)))
        .collect::<Vec<_>>();
    grid.print_with(|point, tile, string| {
        if let Some((index, (segment, _))) = segments_with_points
            .iter()
            .enumerate()
            .find(|(_, (_, points))| points.contains(point))
        {
            let color = *possible_colors
                .get(index % possible_colors.len())
                .unwrap_or(&Color::White);
            let char = possible_strings
                .chars()
                .nth(index % possible_strings.len())
                .unwrap()
                .to_string();

            if path.contains(segment) {
                let _ = write!(string, "{}", char.bold().color(color));
            } else if blacklisted_segments.contains(segment) {
                let _ = write!(string, "{}", char.color(Color::BrightBlack));
            } else {
                let _ = write!(string, "{}", char);
            }
        } else {
            write!(string, "{}", tile).unwrap();
        }
    });
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
struct Segment {
    start_point: Point,
    start_cross_point: Point,
    end_point: Point,
    end_cross_point: Point,
    length: usize,
}

impl Segment {
    fn is_segment(&self, point: &Point) -> bool {
        self.start_point == *point || self.end_point == *point
    }

    fn is_cross_point(&self, point: &Point) -> bool {
        self.start_cross_point == *point || self.end_cross_point == *point
    }

    #[allow(dead_code)]
    fn points(&self, grid: &Array2D<Tile>) -> Vec<Point> {
        let mut current_point = self.start_point;
        let mut points = vec![current_point];

        loop {
            let new_neighbours = current_point
                .neighbours_within_map(grid)
                .into_iter()
                .filter(|new_point| {
                    grid.get_point(new_point)
                        .map(|tile| *tile != Tile::Forest)
                        .unwrap_or(false)
                        && !self.is_cross_point(new_point)
                        && !points.contains(new_point)
                })
                .collect::<Vec<_>>();
            let Some(point) = new_neighbours.first() else {
                break;
            };

            points.push(*point);
            current_point = *point;
        }

        points
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Path,
    Forest,
    Slope { direction: Direction },
}

impl Tile {
    fn can_move_in_direction(&self, direction: Direction, slippery_slopes: bool) -> bool {
        match self {
            Tile::Path => true,
            Tile::Forest => false,
            Tile::Slope {
                direction: slope_direction,
            } => !slippery_slopes || direction == *slope_direction,
        }
    }
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '.' => Tile::Path,
            '#' => Tile::Forest,
            '<' | '>' | '^' | 'v' => Tile::Slope {
                direction: c.into(),
            },
            _ => panic!("Invalid tile: {}", c),
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Path => write!(f, "."),
            Tile::Forest => write!(f, "#"),
            Tile::Slope { direction } => write!(f, "{}", direction),
        }
    }
}
