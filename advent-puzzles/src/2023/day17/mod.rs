use std::{
    collections::{BinaryHeap, HashMap},
    fmt::{Display, Formatter},
};

use array2d::Array2D;
use pathfinding::directed::{astar::astar, dijkstra::dijkstra};

use crate::utils::{
    direction::Direction,
    map::{GenerateMap, InteractWithPoint},
    point::Point,
};

pub fn part1(input: String) -> String {
    let grid = Array2D::generate_map(&input, |_, c| c.to_digit(10).unwrap()).unwrap();

    find_shortest_path(&grid, 1, 3).unwrap().to_string()
}

pub fn part2(input: String) -> String {
    let grid = Array2D::generate_map(&input, |_, c| c.to_digit(10).unwrap()).unwrap();

    find_shortest_path(&grid, 4, 10).unwrap().to_string()
}

fn find_shortest_path(grid: &Array2D<u32>, min_steps: u8, max_steps: u8) -> Option<u32> {
    // dijkstras_manual(grid, min_steps, max_steps)
    dijkstras(grid, min_steps, max_steps)
    // find_with_astar(grid, min_steps, max_steps)
}

fn find_with_astar(grid: &Array2D<u32>, min_steps: u8, max_steps: u8) -> Option<u32> {
    let end_point = Point::new(grid.num_columns() as i32 - 1, grid.num_rows() as i32 - 1);

    let mut heuristics = Array2D::filled_with(0, grid.num_columns(), grid.num_rows());

    for x in (0..grid.num_columns()).rev() {
        for y in (0..grid.num_rows()).rev() {
            let point = Point::new(x as i32, y as i32);
            let cost = *grid.get_point(&point).unwrap();

            if x == grid.num_columns() - 1 && y == grid.num_rows() - 1 {
                heuristics.set_point(&point, cost).unwrap();
                continue;
            }

            let south = heuristics
                .get_point(&point.move_in_direction(Direction::South))
                .map(|v| *v)
                .unwrap_or(u32::MAX);

            let east = heuristics
                .get_point(&point.move_in_direction(Direction::East))
                .map(|v| *v)
                .unwrap_or(u32::MAX);

            let min = south.min(east);

            heuristics.set_point(&point, cost + min).unwrap();
        }
    }

    let (_, c) = astar(
        &PathNode::new(Point::new(0, 0), Direction::South, 0),
        |path_node| {
            let map = |p: PathNode| {
                let cost = *grid.get_point(&p.point).unwrap();
                (p, cost)
            };
            if path_node.steps == 0 {
                return vec![
                    PathNode::new(Point::new(0, 1), Direction::South, 1),
                    PathNode::new(Point::new(1, 0), Direction::East, 1),
                ]
                .into_iter()
                .map(map)
                .into_iter();
            }

            path_node
                .generate_next_nodes(&grid, min_steps, max_steps)
                .into_iter()
                .map(map)
                .into_iter()
        },
        |looking| {
            heuristics
                .get_point(&looking.point)
                .map(|v| *v)
                .unwrap_or(u32::MAX)
        },
        |looking| looking.point == end_point && looking.steps >= min_steps,
    )?;

    Some(c)
}

fn dijkstras(grid: &Array2D<u32>, min_steps: u8, max_steps: u8) -> Option<u32> {
    let end_point = Point::new(grid.num_columns() as i32 - 1, grid.num_rows() as i32 - 1);

    let (_, c) = dijkstra(
        &PathNode::new(Point::new(0, 0), Direction::South, 0),
        |path_node| {
            let map = |p: PathNode| {
                let cost = *grid.get_point(&p.point).unwrap();
                (p, cost)
            };
            if path_node.steps == 0 {
                return vec![
                    PathNode::new(Point::new(0, 1), Direction::South, 1),
                    PathNode::new(Point::new(1, 0), Direction::East, 1),
                ]
                .into_iter()
                .map(map)
                .into_iter();
            }

            path_node
                .generate_next_nodes(&grid, min_steps, max_steps)
                .into_iter()
                .map(map)
                .into_iter()
        },
        |looking| looking.point == end_point && looking.steps >= min_steps,
    )?;

    Some(c)
}

fn dijkstras_manual(grid: &Array2D<u32>, min_steps: u8, max_steps: u8) -> Option<u32> {
    let mut cache: HashMap<PathNode, u32> = HashMap::new();
    let mut heap = BinaryHeap::new();

    let looking_south = TraversalState::new(
        Point::new(0, 1),
        1,
        Direction::South,
        *grid.get_point(&Point::new(0, 1)).unwrap(),
    );
    let looking_east = TraversalState::new(
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

        let next = looking.generate_next_nodes(&grid, &cache, min_steps, max_steps);

        // println!("{} -> {:?} ({})", looking, next, heap.len());
        // print(&grid, &heap, &cache, &looking, &next);
        // std::io::stdin().read_line(&mut String::new()).unwrap();

        heap.extend(next);
    }

    None
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct PathNode {
    point: Point,
    steps: u8,
    direction: Direction,
}

impl PathNode {
    fn new(point: Point, direction: Direction, steps: u8) -> Self {
        Self {
            point,
            steps,
            direction,
        }
    }

    fn derive_from(
        previous: &Self,
        direction: Direction,
        map: &Array2D<u32>,
        min_steps: u8,
        max_steps: u8,
    ) -> Option<Self> {
        if previous.steps == max_steps && previous.direction == direction {
            return None;
        }

        if previous.steps < min_steps && previous.direction != direction {
            return None;
        }

        let point = previous.point.move_in_direction(direction).ok_map(map)?;
        let steps = if previous.direction == direction {
            previous.steps + 1
        } else {
            1
        };

        Some(Self::new(point, direction, steps))
    }

    fn generate_next_nodes(&self, map: &Array2D<u32>, min_steps: u8, max_steps: u8) -> Vec<Self> {
        vec![
            Self::derive_from(self, self.direction.turn_left(), map, min_steps, max_steps),
            Self::derive_from(self, self.direction.turn_right(), map, min_steps, max_steps),
            Self::derive_from(self, self.direction, map, min_steps, max_steps),
        ]
        .into_iter()
        .flatten()
        .collect()
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct TraversalState {
    point: Point,
    steps: u8,
    running_direction: Direction,
    distance: u32,
}

impl TraversalState {
    fn new(point: Point, steps: u8, running_direction: Direction, distance: u32) -> Self {
        Self {
            point,
            steps,
            running_direction,
            distance,
        }
    }

    fn derive_from(
        previous: &Self,
        direction: Direction,
        map: &Array2D<u32>,
        cache: &HashMap<PathNode, u32>,
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
    fn generate_next_nodes(
        &self,
        map: &Array2D<u32>,
        cache: &HashMap<PathNode, u32>,
        min_steps: u8,
        max_steps: u8,
    ) -> Vec<Self> {
        vec![
            Self::derive_from(
                self,
                self.running_direction.turn_left(),
                map,
                cache,
                min_steps,
                max_steps,
            ),
            Self::derive_from(
                self,
                self.running_direction.turn_right(),
                map,
                cache,
                min_steps,
                max_steps,
            ),
            Self::derive_from(
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

impl From<TraversalState> for PathNode {
    fn from(looking: TraversalState) -> Self {
        Self {
            point: looking.point,
            steps: looking.steps,
            direction: looking.running_direction,
        }
    }
}

impl PartialOrd for TraversalState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for TraversalState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .distance
            .cmp(&self.distance)
            .then_with(|| other.point.cmp(&self.point))
    }
}

impl Display for TraversalState {
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
    heap: &BinaryHeap<TraversalState>,
    cache: &HashMap<PathNode, u32>,
    looking: &TraversalState,
    next: &[TraversalState],
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
