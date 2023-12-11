use std::fmt::Display;

use array2d::Array2D;

use super::direction::Direction;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

    pub fn ok_dimensions(&self, width: usize, height: usize) -> Option<Self> {
        if self.x >= 0 && self.y >= 0 && self.x < width as i32 && self.y < height as i32 {
            Some(*self)
        } else {
            None
        }
    }

    pub fn ok_map<T>(&self, map: &Array2D<T>) -> Option<Self> {
        self.ok_dimensions(map.num_columns(), map.num_rows())
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
            .filter(|p| p.ok_dimensions(map.num_columns(), map.num_rows()).is_some())
            .collect()
    }

    pub fn neighbours_within_map_all_directions<T>(&self, map: &Array2D<T>) -> Vec<Point> {
        self.neighbours_all_directions()
            .into_iter()
            .filter(|p| p.ok_dimensions(map.num_columns(), map.num_rows()).is_some())
            .collect()
    }

    pub fn manhattan_distance(&self, other: &Point) -> usize {
        ((self.x - other.x).abs() + (self.y - other.y).abs()) as usize
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
