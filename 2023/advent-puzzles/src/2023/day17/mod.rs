use array2d::Array2D;
use pathfinding::directed::dijkstra::dijkstra;

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
