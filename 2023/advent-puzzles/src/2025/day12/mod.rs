use std::collections::HashMap;

use array2d::Array2D;
use nom::{
    character::complete::{char, digit1, line_ending, one_of, space1},
    combinator::{map, map_res},
    multi::{many1, separated_list0, separated_list1},
    sequence::{preceded, separated_pair, terminated, tuple},
    IResult,
};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::utils::point::Point;

#[derive(Debug, Clone)]
pub struct ParsedShape {
    pub index: usize,
    pub grid: Vec<Vec<bool>>,
}

#[derive(Debug, Clone)]
pub struct Region {
    pub width: usize,
    pub length: usize,
    pub present_counts: Vec<usize>,
}

#[derive(Debug, Clone)]
pub struct Input {
    pub shapes: Vec<ParsedShape>,
    pub regions: Vec<Region>,
}

fn parse_usize(input: &str) -> IResult<&str, usize> {
    map_res(digit1, |s: &str| s.parse::<usize>())(input)
}

fn parse_shape_row(input: &str) -> IResult<&str, Vec<bool>> {
    map(many1(one_of("#.")), |chars| {
        chars.into_iter().map(|c| c == '#').collect()
    })(input)
}

fn parse_shape(input: &str) -> IResult<&str, ParsedShape> {
    let (input, index) = terminated(parse_usize, tuple((char(':'), line_ending)))(input)?;
    let (input, grid) = separated_list1(line_ending, parse_shape_row)(input)?;
    Ok((input, ParsedShape { index, grid }))
}

fn parse_shapes(input: &str) -> IResult<&str, Vec<ParsedShape>> {
    separated_list1(many1(line_ending), parse_shape)(input)
}

fn parse_dimensions(input: &str) -> IResult<&str, (usize, usize)> {
    terminated(
        separated_pair(parse_usize, char('x'), parse_usize),
        char(':'),
    )(input)
}

fn parse_region(input: &str) -> IResult<&str, Region> {
    let (input, (width, length)) = parse_dimensions(input)?;
    let (input, present_counts) = preceded(space1, separated_list0(space1, parse_usize))(input)?;
    Ok((
        input,
        Region {
            width,
            length,
            present_counts,
        },
    ))
}

fn parse_regions(input: &str) -> IResult<&str, Vec<Region>> {
    separated_list1(line_ending, parse_region)(input)
}

pub fn parse_input(input: &str) -> IResult<&str, Input> {
    let (input, shapes) = parse_shapes(input)?;
    let (input, _) = many1(line_ending)(input)?;
    let (input, regions) = parse_regions(input)?;
    Ok((input, Input { shapes, regions }))
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Shape {
    occupied_spaces: usize,
    orientation: Vec<ShapeOrientation>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct ShapeOrientation {
    /// Relative offsets from top-left for occupied cells
    offsets: Vec<Point>,
    /// Width of the bounding box
    width: usize,
    /// Height of the bounding box
    height: usize,
}

impl ShapeOrientation {
    fn from_grid(grid: &[Vec<bool>]) -> Self {
        let height = grid.len();
        let width = if height > 0 { grid[0].len() } else { 0 };

        let mut offsets = Vec::new();
        for (r, row) in grid.iter().enumerate() {
            for (c, &occupied) in row.iter().enumerate() {
                if occupied {
                    offsets.push(Point::new(c as i32, r as i32));
                }
            }
        }

        ShapeOrientation {
            offsets,
            width,
            height,
        }
    }

    fn is_same(&self, other: &ShapeOrientation) -> bool {
        self.width == other.width && self.height == other.height && self.offsets == other.offsets
    }
}

impl Shape {
    fn new(parsed_shape: &ParsedShape) -> Self {
        let grid = parsed_shape.grid.clone();
        let occupied_spaces = grid.iter().flatten().filter(|&&b| b).count();

        let mut orientations = Vec::new();

        let mut current = grid.clone();

        for _ in 0..4 {
            let orientation = ShapeOrientation::from_grid(&current);
            if !orientations
                .iter()
                .any(|o: &ShapeOrientation| o.is_same(&orientation))
            {
                orientations.push(orientation);
            }
            current = rotate_90(&current);
        }

        current = flip_horizontal(&grid);

        for _ in 0..4 {
            let orientation = ShapeOrientation::from_grid(&current);
            if !orientations
                .iter()
                .any(|o: &ShapeOrientation| o.is_same(&orientation))
            {
                orientations.push(orientation);
            }
            current = rotate_90(&current);
        }

        Shape {
            occupied_spaces,
            orientation: orientations,
        }
    }
}

fn rotate_90(grid: &[Vec<bool>]) -> Vec<Vec<bool>> {
    if grid.is_empty() {
        return vec![];
    }
    let rows = grid.len();
    let cols = grid[0].len();
    let mut rotated = vec![vec![false; rows]; cols];
    for r in 0..rows {
        for c in 0..cols {
            rotated[c][rows - 1 - r] = grid[r][c];
        }
    }
    rotated
}

fn flip_horizontal(grid: &[Vec<bool>]) -> Vec<Vec<bool>> {
    grid.iter()
        .map(|row| row.iter().rev().cloned().collect())
        .collect()
}

pub fn part1(input: String) -> String {
    let (_, input) = parse_input(&input).expect("Could not parse input");

    let regions = input.regions;
    let shapes = input
        .shapes
        .iter()
        .map(|s| Shape::new(s))
        .collect::<Vec<_>>();

    regions
        .par_iter()
        .filter(|r| {
            let mut grid = Array2D::filled_with(false, r.length, r.width);
            let mut to_place = r
                .present_counts
                .iter()
                .enumerate()
                .filter(|(_, n)| **n > 0)
                .map(|(i, n)| (&shapes[i], *n))
                .collect::<HashMap<&Shape, usize>>();

            is_placement_possible(&mut grid, r.length * r.width, &mut to_place)
        })
        .count()
        .to_string()
}

fn is_placement_possible(
    grid: &mut Array2D<bool>,
    spots_left: usize,
    to_place: &mut HashMap<&Shape, usize>,
) -> bool {
    if to_place.is_empty() {
        return true;
    }

    if spots_left == 0 {
        return false;
    }

    let spots_needed = to_place
        .iter()
        .map(|(shape, count)| shape.occupied_spaces * count)
        .sum::<usize>();
    if spots_needed > spots_left {
        return false;
    }

    let shapes_to_try: Vec<&Shape> = to_place.keys().cloned().collect();

    for shape in shapes_to_try {
        let mut any_orientation_placed = false;

        for orientation in &shape.orientation {
            let Some(placed_points) = try_place_orientation(grid, orientation) else {
                continue;
            };
            any_orientation_placed = true;

            let count = to_place.get_mut(shape).unwrap();
            *count -= 1;
            let removed = if *count == 0 {
                to_place.remove(shape);
                true
            } else {
                false
            };

            let new_spots_left = spots_left - shape.occupied_spaces;

            if is_placement_possible(grid, new_spots_left, to_place) {
                return true;
            }

            if removed {
                to_place.insert(shape, 1);
            } else {
                *to_place.get_mut(shape).unwrap() += 1;
            }

            remove_placement(grid, &placed_points);
        }

        // If this shape couldn't be placed in any orientation, we can never complete the puzzle
        if !any_orientation_placed {
            return false;
        }
    }

    return false;
}

fn try_place_orientation(
    grid: &mut Array2D<bool>,
    orientation: &ShapeOrientation,
) -> Option<Vec<Point>> {
    use crate::utils::map::InteractWithPoint;

    let shape_height = orientation.height;
    let shape_width = orientation.width;

    let grid_height = grid.num_rows();
    let grid_width = grid.num_columns();

    if shape_height > grid_height || shape_width > grid_width {
        return None;
    }

    for start_row in 0..=(grid_height - shape_height) {
        for start_col in 0..=(grid_width - shape_width) {
            let can_place = orientation.offsets.iter().all(|offset| {
                let point = Point::new(start_col as i32 + offset.x, start_row as i32 + offset.y);
                matches!(grid.get_point(&point), Some(&false))
            });

            if can_place {
                // Place the shape and collect the points
                let mut placed_points = Vec::with_capacity(orientation.offsets.len());
                for offset in &orientation.offsets {
                    let point =
                        Point::new(start_col as i32 + offset.x, start_row as i32 + offset.y);
                    grid.set_point(&point, true).expect("Point should be valid");
                    placed_points.push(point);
                }
                return Some(placed_points);
            }
        }
    }

    None
}

fn remove_placement(grid: &mut Array2D<bool>, points: &[Point]) {
    use crate::utils::map::InteractWithPoint;

    for point in points {
        grid.set_point(point, false).expect("Point should be valid");
    }
}

pub fn part2(_input: String) -> String {
    todo!()
}
