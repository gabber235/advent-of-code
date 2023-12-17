use std::{
    collections::{BinaryHeap, HashMap},
    fmt::{Display, Formatter},
};

use array2d::Array2D;

use crate::utils::{
    direction::Direction,
    map::{GenerateMap, InteractWithPoint},
    point::Point,
};

pub fn part1(input: String) -> String {
    let grid = Array2D::generate_map(&input, |_, c| c.to_digit(10).unwrap()).unwrap();

    dijkstras(&grid, 1, 3).unwrap().to_string()
}

pub fn part2(input: String) -> String {
    let grid = Array2D::generate_map(&input, |_, c| c.to_digit(10).unwrap()).unwrap();

    dijkstras(&grid, 4, 10).unwrap().to_string()
}

fn dijkstras(grid: &Array2D<u32>, min_steps: u8, max_steps: u8) -> Option<u32> {
    let mut cache: HashMap<Cache, u32> = HashMap::new();
    let mut heap = BinaryHeap::new();

    let looking_south = Looking::new(
        Point::new(0, 1),
        1,
        Direction::South,
        *grid.get_point(&Point::new(0, 1)).unwrap(),
    );
    let looking_east = Looking::new(
        Point::new(1, 0),
        1,
        Direction::East,
        *grid.get_point(&Point::new(1, 0)).unwrap(),
    );

    heap.push(looking_south);
    heap.push(looking_east);

    let end_point = Point::new(grid.num_columns() as i32 - 1, grid.num_rows() as i32 - 1);

    while let Some(looking) = heap.pop() {
        if looking.point == end_point && looking.steps >= min_steps {
            return Some(looking.distance);
        }

        if cache
            .get(&looking.into())
            .is_some_and(|&v| v <= looking.distance)
        {
            continue;
        }

        cache.insert(looking.clone().into(), looking.distance);

        let next = looking.next(&grid, &cache, min_steps, max_steps);

        // println!("{} -> {:?} ({})", looking, next, heap.len());
        // print(&grid, &heap, &cache, &looking, &next);
        // std::io::stdin().read_line(&mut String::new()).unwrap();

        heap.extend(next);
    }

    None
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Cache {
    point: Point,
    steps: u8,
    direction: Direction,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Looking {
    point: Point,
    steps: u8,
    running_direction: Direction,
    distance: u32,
}

impl Looking {
    fn new(point: Point, steps: u8, running_direction: Direction, distance: u32) -> Self {
        Self {
            point,
            steps,
            running_direction,
            distance,
        }
    }

    fn from_previous(
        previous: &Self,
        direction: Direction,
        map: &Array2D<u32>,
        cache: &HashMap<Cache, u32>,
        min_steps: u8,
        max_steps: u8,
    ) -> Option<Self> {
        if previous.steps == max_steps && previous.running_direction == direction {
            return None;
        }

        if previous.steps < min_steps && previous.running_direction != direction {
            return None;
        }

        let point = previous.point.move_in_direction(direction).ok_map(map)?;
        let distance = previous.distance + map.get_point(&point).unwrap();

        let running_strait = if previous.running_direction == direction {
            previous.steps + 1
        } else {
            1
        };

        let new = Self::new(point, running_strait, direction, distance);

        if cache.get(&new.into()).is_some_and(|&v| v <= distance) {
            return None;
        }

        Some(new)
    }

    /// If the running strait is 3, then we can only go left or right
    /// We can't go back
    fn next(
        &self,
        map: &Array2D<u32>,
        cache: &HashMap<Cache, u32>,
        min_steps: u8,
        max_steps: u8,
    ) -> Vec<Self> {
        vec![
            Self::from_previous(
                self,
                self.running_direction.turn_left(),
                map,
                cache,
                min_steps,
                max_steps,
            ),
            Self::from_previous(
                self,
                self.running_direction.turn_right(),
                map,
                cache,
                min_steps,
                max_steps,
            ),
            Self::from_previous(
                self,
                self.running_direction,
                map,
                cache,
                min_steps,
                max_steps,
            ),
        ]
        .into_iter()
        .flatten()
        .collect()
    }
}

impl From<Looking> for Cache {
    fn from(looking: Looking) -> Self {
        Self {
            point: looking.point,
            steps: looking.steps,
            direction: looking.running_direction,
        }
    }
}

impl PartialOrd for Looking {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Looking {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .distance
            .cmp(&self.distance)
            .then_with(|| other.point.cmp(&self.point))
    }
}

impl Display for Looking {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Looking at {} with running {} in {} with distance {}",
            self.point, self.steps, self.running_direction, self.distance
        )
    }
}

fn print(
    grid: &Array2D<u32>,
    heap: &BinaryHeap<Looking>,
    cache: &HashMap<Cache, u32>,
    looking: &Looking,
    next: &[Looking],
) {
    for y in 0..grid.num_rows() {
        for x in 0..grid.num_columns() {
            let point = Point::new(x as i32, y as i32);

            if point == looking.point {
                print!("{}", looking.running_direction);
            } else if next.iter().any(|l| l.point == point) {
                print!("o");
            } else if heap.iter().any(|l| l.point == point) {
                print!("x");
            } else if cache.iter().find(|(k, _)| k.point == point).is_some() {
                print!("#");
            } else {
                print!(".");
            }
        }

        println!();
    }
}
