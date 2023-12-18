use std::{fmt::Display, i32};

use array2d::Array2D;

use super::direction::Direction;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn from_index(index: usize, width: usize) -> Self {
        Self {
            x: (index % width) as i32,
            y: (index / width) as i32,
        }
    }

    pub fn index(&self, width: usize) -> usize {
        (self.y as usize) * width + (self.x as usize)
    }

    pub fn ok(&self) -> Option<Self> {
        if self.x >= 0 && self.y >= 0 {
            Some(*self)
        } else {
            None
        }
    }

    pub fn ok_dimensions(&self, min_x: i32, min_y: i32, max_x: i32, max_y: i32) -> Option<Self> {
        if self.x >= min_x && self.y >= min_y && self.x <= max_x && self.y <= max_y {
            Some(*self)
        } else {
            None
        }
    }

    pub fn ok_map<T>(&self, map: &Array2D<T>) -> Option<Self> {
        self.ok_dimensions(
            0,
            0,
            map.num_columns() as i32 - 1,
            map.num_rows() as i32 - 1,
        )
    }

    pub fn move_in_direction(&self, direction: Direction) -> Point {
        self.move_n_in_direction(direction, 1)
    }

    pub fn move_n_in_direction(&self, direction: Direction, n: i32) -> Point {
        match direction {
            Direction::North => Self::new(self.x, self.y - n),
            Direction::East => Self::new(self.x + n, self.y),
            Direction::South => Self::new(self.x, self.y + n),
            Direction::West => Self::new(self.x - n, self.y),
        }
    }

    pub fn neighbours(&self) -> Vec<Point> {
        vec![
            self.move_in_direction(Direction::North),
            self.move_in_direction(Direction::East),
            self.move_in_direction(Direction::South),
            self.move_in_direction(Direction::West),
        ]
        .into_iter()
        .map(|p| p)
        .collect()
    }

    pub fn neighbours_all_directions(&self) -> Vec<Point> {
        vec![
            Self::new(self.x - 1, self.y - 1),
            Self::new(self.x, self.y - 1),
            Self::new(self.x + 1, self.y - 1),
            Self::new(self.x - 1, self.y),
            Self::new(self.x + 1, self.y),
            Self::new(self.x - 1, self.y + 1),
            Self::new(self.x, self.y + 1),
            Self::new(self.x + 1, self.y + 1),
        ]
    }

    pub fn neighbours_within_map<T>(&self, map: &Array2D<T>) -> Vec<Point> {
        self.neighbours()
            .into_iter()
            .filter(|p| p.ok_map(map).is_some())
            .collect()
    }

    pub fn neighbours_within_map_all_directions<T>(&self, map: &Array2D<T>) -> Vec<Point> {
        self.neighbours_all_directions()
            .into_iter()
            .filter(|p| p.ok_map(map).is_some())
            .collect()
    }

    pub fn neighbours_within_dimensions(
        &self,
        min_x: i32,
        min_y: i32,
        max_x: i32,
        max_y: i32,
    ) -> Vec<Point> {
        self.neighbours()
            .into_iter()
            .filter(|p| p.ok_dimensions(min_x, min_y, max_x, max_y).is_some())
            .collect()
    }

    pub fn neighbours_within_dimensions_all_directions(
        &self,
        min_x: i32,
        min_y: i32,
        max_x: i32,
        max_y: i32,
    ) -> Vec<Point> {
        self.neighbours_all_directions()
            .into_iter()
            .filter(|p| p.ok_dimensions(min_x, min_y, max_x, max_y).is_some())
            .collect()
    }

    pub fn manhattan_distance(&self, other: &Point) -> usize {
        ((self.x - other.x).abs() + (self.y - other.y).abs()) as usize
    }

    pub fn is_origin(&self) -> bool {
        self.x == 0 && self.y == 0
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl From<(i32, i32)> for Point {
    fn from((x, y): (i32, i32)) -> Self {
        Self::new(x, y)
    }
}

impl From<Point> for (i32, i32) {
    fn from(point: Point) -> Self {
        (point.x, point.y)
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.y.cmp(&other.y).then_with(|| self.x.cmp(&other.x))
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
