use array2d::Array2D;
use colored::Colorize;
use std::fmt::Display;

pub fn part1(input: String) -> String {
    let mut grid = part1::parse(&input);
    // part1::print_grid(&grid);
    let distance = part1::walk_grid(&mut grid);
    distance.to_string()
}

mod part1 {
    use super::*;
    pub(crate) fn walk_grid(grid: &mut Array2D<Pipe>) -> usize {
        let start_point = find_start(&grid);
        let (mut point1, mut point2) = find_connected_to_start(&grid, &start_point);
        let mut distance = 0;
        loop {
            // print_grid(grid);
            // println!("\n");
            distance += 1;
            update_pipe(grid, point1, distance);
            update_pipe(grid, point2, distance);

            let Some(next_point1) = find_next_point(&grid, &point1) else {
                break;
            };

            let Some(next_point2) = find_next_point(&grid, &point2) else {
                break;
            };

            point1 = next_point1;
            point2 = next_point2;
        }

        distance
    }

    fn update_pipe(grid: &mut Array2D<Pipe>, point: Point, distance: usize) {
        let pipe_distance = grid.get_point_mut(&point).unwrap();
        pipe_distance.distance = distance;
    }

    fn find_next_point(grid: &Array2D<Pipe>, point: &Point) -> Option<Point> {
        let mut connected = Vec::new();
        let pipe = grid.get_point(&point).unwrap();
        for direction in pipe.shape.directions() {
            let Some(point) = point.move_in_direction(direction) else {
                continue;
            };
            if grid
                .get_point(&point)
                .unwrap()
                .shape
                .is_connected(direction.opposite())
            {
                connected.push(point);
            }
        }
        if connected.len() != 2 {
            panic!("Point has {} connections", connected.len());
        }
        let first = grid.get_point(&connected[0]).unwrap();
        if first.distance == 0 && first.shape != PipeShape::Start {
            return Some(connected[0]);
        }
        let second = grid.get_point(&connected[1]).unwrap();
        if second.distance == 0 && second.shape != PipeShape::Start {
            return Some(connected[1]);
        }
        None
    }

    fn find_connected_to_start(grid: &Array2D<Pipe>, start_point: &Point) -> (Point, Point) {
        let mut connected = Vec::new();
        for direction in &[
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ] {
            let Some(point) = start_point.move_in_direction(*direction) else {
                continue;
            };
            let pipe = grid.get_point(&point).unwrap();
            if pipe.shape.is_connected(direction.opposite()) {
                connected.push(point);
            }
        }

        if connected.len() != 2 {
            panic!(
                "Start point has {} connections ({:?})",
                connected.len(),
                connected
            );
        }
        (connected[0], connected[1])
    }

    fn find_start(grid: &Array2D<Pipe>) -> Point {
        for (y, row) in grid.rows_iter().enumerate() {
            for (x, pipe) in row.enumerate() {
                if pipe.shape == PipeShape::Start {
                    return Point::new(x, y);
                }
            }
        }
        panic!("No start found");
    }

    pub fn print_grid(grid: &Array2D<Pipe>) {
        for row in grid.rows_iter() {
            for pipe in row {
                print!("{}", pipe);
            }
            println!();
        }
    }

    pub(crate) fn parse(input: &str) -> Array2D<Pipe> {
        let vec: Vec<Vec<Pipe>> = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| PipeShape::from(c))
                    .map(|pipe| Pipe::new(pipe, 0))
                    .collect()
            })
            .collect();

        Array2D::from_rows(&vec).unwrap()
    }

    #[derive(Debug, Clone)]
    pub struct Pipe {
        shape: PipeShape,
        distance: usize,
    }

    impl Pipe {
        fn new(pipe: PipeShape, distance: usize) -> Self {
            Pipe {
                shape: pipe,
                distance,
            }
        }
    }

    impl Display for Pipe {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if self.distance > 0 {
                write!(f, "{:1}", self.distance)
            } else {
                write!(f, "{}", self.shape)
            }
        }
    }
}

pub fn part2(input: String) -> String {
    let mut grid = part2::parse(&input);
    part2::walk_grid(&mut grid);
    // part2::print_grid(&grid);
    let mut grid = part2::expand_grid(&grid);
    part2::flood_grid(&mut grid);

    // part2::print_grid(&grid);

    grid.rows_iter()
        .map(|row| {
            row.filter(|pipe| pipe.state == part2::PipeState::Unknown && !pipe.fake)
                .count()
        })
        .sum::<usize>()
        .to_string()
}

mod part2 {
    use super::*;

    pub fn flood_grid(grid: &mut Array2D<Pipe>) {
        let mut left_points = vec![Point::new(0, 0)];

        while !left_points.is_empty() {
            let point = left_points.pop().unwrap();
            let pipe = grid.get_point(&point).unwrap();
            match pipe.state {
                PipeState::Unknown => {
                    grid.get_point_mut(&point).unwrap().state = PipeState::OutsideLoop;
                }
                _ => continue,
            }

            for neighbour in point.neighbours() {
                let Some(pipe) = grid.get_point(&neighbour) else {
                    continue;
                };
                if pipe.state != PipeState::Unknown {
                    continue;
                }
                left_points.push(neighbour);
            }
        }
    }

    pub fn expand_grid(grid: &Array2D<Pipe>) -> Array2D<Pipe> {
        let mut new_grid = Array2D::filled_with(
            Pipe {
                shape: PipeShape::Empty,
                state: PipeState::Unknown,
                fake: true,
            },
            grid.num_rows() * 3,
            grid.num_columns() * 3,
        );

        for (y, row) in grid.rows_iter().enumerate() {
            for (x, pipe) in row.enumerate() {
                let new_x = x * 3;
                let new_y = y * 3;
                new_grid[(new_y + 1, new_x + 1)] = pipe.clone();

                let mut pipe = pipe.clone();

                if pipe.shape == PipeShape::Start {
                    pipe.shape = find_start_shape(&grid, &Point::new(x, y));
                }

                for (y_offset, row) in pipe.enhance().rows_iter().enumerate() {
                    for (x_offset, pipe) in row.enumerate() {
                        new_grid[(new_y + y_offset, new_x + x_offset)] = pipe.clone();
                    }
                }
            }
        }

        new_grid
    }

    pub(crate) fn walk_grid(grid: &mut Array2D<Pipe>) {
        let start_point = find_start(&grid);

        grid.get_point_mut(&start_point).unwrap().state = PipeState::PartOfLoop;

        let (mut point1, mut point2) = find_connected_to_start(&grid, &start_point);
        loop {
            update_pipe(grid, point1);
            update_pipe(grid, point2);

            let Some(next_point1) = find_next_point(&grid, &point1) else {
                break;
            };

            let Some(next_point2) = find_next_point(&grid, &point2) else {
                break;
            };

            point1 = next_point1;
            point2 = next_point2;
        }
    }

    fn update_pipe(grid: &mut Array2D<Pipe>, point: Point) {
        let pipe = grid.get_point_mut(&point).unwrap();
        pipe.state = PipeState::PartOfLoop;
    }

    fn find_next_point(grid: &Array2D<Pipe>, point: &Point) -> Option<Point> {
        let mut connected = Vec::new();
        let pipe = grid.get_point(&point).unwrap();
        for direction in pipe.shape.directions() {
            let Some(point) = point.move_in_direction(direction) else {
                continue;
            };
            if grid
                .get_point(&point)
                .unwrap()
                .shape
                .is_connected(direction.opposite())
            {
                connected.push(point);
            }
        }
        if connected.len() != 2 {
            panic!("Point has {} connections", connected.len());
        }
        let first = grid.get_point(&connected[0]).unwrap();
        if first.state == PipeState::Unknown {
            return Some(connected[0]);
        }
        let second = grid.get_point(&connected[1]).unwrap();
        if second.state == PipeState::Unknown {
            return Some(connected[1]);
        }
        None
    }

    fn find_start_shape(grid: &Array2D<Pipe>, start_point: &Point) -> PipeShape {
        let directions = find_directions_start(&grid, &start_point);
        let (direction1, direction2) = (directions[0], directions[1]);
        PipeShape::from_directions(&[direction1, direction2])
    }

    fn find_directions_start(grid: &Array2D<Pipe>, start_point: &Point) -> Vec<Direction> {
        let mut directions = Vec::new();
        for direction in &[
            Direction::North,
            Direction::South,
            Direction::East,
            Direction::West,
        ] {
            let Some(point) = start_point.move_in_direction(*direction) else {
                continue;
            };
            let pipe = grid.get_point(&point).unwrap();
            if pipe.shape.is_connected(direction.opposite()) {
                directions.push(*direction);
            }
        }

        if directions.len() != 2 {
            panic!(
                "Start point has {} connections ({:?})",
                directions.len(),
                directions
            );
        }
        directions
    }

    fn find_connected_to_start(grid: &Array2D<Pipe>, start_point: &Point) -> (Point, Point) {
        let directions = find_directions_start(&grid, &start_point);
        let mut connected = Vec::new();
        for direction in directions {
            let Some(point) = start_point.move_in_direction(direction) else {
                continue;
            };
            let pipe = grid.get_point(&point).unwrap();
            if pipe.shape.is_connected(direction.opposite()) {
                connected.push(point);
            }
        }

        if connected.len() != 2 {
            panic!(
                "Start point has {} connections ({:?})",
                connected.len(),
                connected
            );
        }

        (connected[0], connected[1])
    }

    fn find_start(grid: &Array2D<Pipe>) -> Point {
        for (y, row) in grid.rows_iter().enumerate() {
            for (x, pipe) in row.enumerate() {
                if pipe.shape == PipeShape::Start {
                    return Point::new(x, y);
                }
            }
        }
        panic!("No start found");
    }

    pub(crate) fn parse(input: &str) -> Array2D<Pipe> {
        let vec: Vec<Vec<Pipe>> = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| PipeShape::from(c))
                    .map(|pipe| Pipe::new(pipe))
                    .collect()
            })
            .collect();

        Array2D::from_rows(&vec).unwrap()
    }

    pub fn print_grid(grid: &Array2D<Pipe>) {
        for row in grid.rows_iter() {
            for pipe in row {
                print!("{}", pipe);
            }
            println!();
        }
    }

    #[derive(Debug, Clone)]
    pub struct Pipe {
        pub shape: PipeShape,
        pub state: PipeState,
        pub fake: bool,
    }

    impl Pipe {
        fn new(pipe: PipeShape) -> Self {
            Pipe {
                shape: pipe,
                state: PipeState::Unknown,
                fake: false,
            }
        }
    }

    impl Display for Pipe {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if self.fake {
                return write!(f, "{}", format!("{}", self.shape).dimmed());
            }
            match self.state {
                PipeState::PartOfLoop => write!(f, "{}", format!("{}", self.shape).red()),
                PipeState::OutsideLoop => write!(f, "{}", format!("{}", self.shape).green()),
                PipeState::Unknown => write!(f, "{}", format!("{}", self.shape).yellow()),
            }
        }
    }

    impl Pipe {
        fn enhance(&self) -> Array2D<Pipe> {
            let mut grid = Array2D::filled_with(
                Pipe {
                    shape: PipeShape::Empty,
                    state: PipeState::Unknown,
                    fake: true,
                },
                3,
                3,
            );

            fn set(grid: &mut Array2D<Pipe>, current: &Pipe, x: usize, y: usize, pipe: PipeShape) {
                grid[(y, x)] = Pipe {
                    shape: pipe,
                    state: current.state,
                    fake: true,
                };
            }
            grid[(1, 1)] = self.clone();

            match self.shape {
                PipeShape::Empty => {}
                PipeShape::Start => {}
                PipeShape::Vertical => {
                    set(&mut grid, self, 1, 0, PipeShape::Vertical);
                    set(&mut grid, self, 1, 2, PipeShape::Vertical);
                }
                PipeShape::Horizontal => {
                    set(&mut grid, self, 0, 1, PipeShape::Horizontal);
                    set(&mut grid, self, 2, 1, PipeShape::Horizontal);
                }
                PipeShape::NorthEast => {
                    set(&mut grid, self, 1, 0, PipeShape::Vertical);
                    set(&mut grid, self, 2, 1, PipeShape::Horizontal);
                }
                PipeShape::NorthWest => {
                    set(&mut grid, self, 1, 0, PipeShape::Vertical);
                    set(&mut grid, self, 0, 1, PipeShape::Horizontal);
                }
                PipeShape::SouthEast => {
                    set(&mut grid, self, 1, 2, PipeShape::Vertical);
                    set(&mut grid, self, 2, 1, PipeShape::Horizontal);
                }
                PipeShape::SouthWest => {
                    set(&mut grid, self, 1, 2, PipeShape::Vertical);
                    set(&mut grid, self, 0, 1, PipeShape::Horizontal);
                }
            }

            grid
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum PipeState {
        PartOfLoop,
        OutsideLoop,
        Unknown,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PipeShape {
    Empty,
    Start,
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
}

impl PipeShape {
    fn from_directions(directions: &[Direction]) -> Self {
        match directions {
            [Direction::North, Direction::East] => PipeShape::NorthEast,
            [Direction::North, Direction::West] => PipeShape::NorthWest,
            [Direction::South, Direction::East] => PipeShape::SouthEast,
            [Direction::South, Direction::West] => PipeShape::SouthWest,
            [Direction::North, Direction::South] => PipeShape::Vertical,
            [Direction::East, Direction::West] => PipeShape::Horizontal,
            _ => panic!("Invalid directions: {:?}", directions),
        }
    }

    fn is_connected(&self, direction: Direction) -> bool {
        match self {
            PipeShape::Empty => false,
            PipeShape::Start => true,
            PipeShape::Vertical => direction == Direction::North || direction == Direction::South,
            PipeShape::Horizontal => direction == Direction::East || direction == Direction::West,
            PipeShape::NorthEast => direction == Direction::North || direction == Direction::East,
            PipeShape::NorthWest => direction == Direction::North || direction == Direction::West,
            PipeShape::SouthEast => direction == Direction::South || direction == Direction::East,
            PipeShape::SouthWest => direction == Direction::South || direction == Direction::West,
        }
    }

    fn directions(&self) -> Vec<Direction> {
        match self {
            PipeShape::Empty => Vec::new(),
            PipeShape::Start => vec![
                Direction::North,
                Direction::East,
                Direction::South,
                Direction::West,
            ],
            PipeShape::Vertical => vec![Direction::North, Direction::South],
            PipeShape::Horizontal => vec![Direction::East, Direction::West],
            PipeShape::NorthEast => vec![Direction::North, Direction::East],
            PipeShape::NorthWest => vec![Direction::North, Direction::West],
            PipeShape::SouthEast => vec![Direction::South, Direction::East],
            PipeShape::SouthWest => vec![Direction::South, Direction::West],
        }
    }
}

impl From<char> for PipeShape {
    fn from(c: char) -> Self {
        match c {
            '.' => PipeShape::Empty,
            '|' => PipeShape::Vertical,
            '-' => PipeShape::Horizontal,
            'S' => PipeShape::Start,
            'L' => PipeShape::NorthEast,
            'J' => PipeShape::NorthWest,
            '7' => PipeShape::SouthWest,
            'F' => PipeShape::SouthEast,
            _ => panic!("Invalid pipe character: {}", c),
        }
    }
}

impl Display for PipeShape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            PipeShape::Empty => '.',
            PipeShape::Start => 'S',
            PipeShape::Vertical => '│',
            PipeShape::Horizontal => '─',
            PipeShape::NorthEast => '└',
            PipeShape::NorthWest => '┘',
            PipeShape::SouthWest => '┐',
            PipeShape::SouthEast => '┌',
        };
        write!(f, "{}", c)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn opposite(&self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Point { x, y }
    }

    fn move_in_direction(&self, direction: Direction) -> Option<Point> {
        match direction {
            Direction::North => {
                if self.y == 0 {
                    None
                } else {
                    Some(Point::new(self.x, self.y - 1))
                }
            }
            Direction::East => Some(Point::new(self.x + 1, self.y)),
            Direction::South => Some(Point::new(self.x, self.y + 1)),
            Direction::West => {
                if self.x == 0 {
                    None
                } else {
                    Some(Point::new(self.x - 1, self.y))
                }
            }
        }
    }

    fn neighbours(&self) -> Vec<Point> {
        vec![
            self.move_in_direction(Direction::North),
            self.move_in_direction(Direction::East),
            self.move_in_direction(Direction::South),
            self.move_in_direction(Direction::West),
        ]
        .into_iter()
        .filter_map(|p| p)
        .collect()
    }
}

trait GetWithPoint {
    type Item;
    fn get_point(&self, point: &Point) -> Option<&Self::Item>;
    fn get_point_mut(&mut self, point: &Point) -> Option<&mut Self::Item>;
}

impl<T> GetWithPoint for Array2D<T> {
    type Item = T;

    fn get_point(&self, point: &Point) -> Option<&Self::Item> {
        self.get(point.y, point.x)
    }

    fn get_point_mut(&mut self, point: &Point) -> Option<&mut Self::Item> {
        self.get_mut(point.y, point.x)
    }
}
