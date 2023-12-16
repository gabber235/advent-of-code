use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn opposite(&self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }
}

impl From<char> for Direction {
    fn from(c: char) -> Self {
        match c {
            '^' => Self::North,
            '>' => Self::East,
            'v' => Self::South,
            '<' => Self::West,
            _ => panic!("Invalid direction: {}", c),
        }
    }
}

impl From<Direction> for char {
    fn from(d: Direction) -> Self {
        match d {
            Direction::North => '^',
            Direction::East => '>',
            Direction::South => 'v',
            Direction::West => '<',
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c: char = (*self).into();
        write!(f, "{}", c)
    }
}
