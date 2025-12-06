use std::u16;

use nom::{bytes::complete::tag, character::complete::newline, multi::separated_list1, IResult};
use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};

use crate::utils::point_3d::Point3D;

pub fn part1(input: String) -> String {
    let mut bricks = parse_bricks(&input).unwrap().1;
    bricks.sort();
    settle_bricks(&mut bricks);
    let mut count = 0;

    for i in 0..bricks.len() {
        let brick = bricks.remove(i);
        // If there is any other brick now that can fall, we can't remove this one
        if !bricks.par_iter().any(|brick| brick.can_fall(&bricks)) {
            count += 1;
        }
        bricks.insert(i, brick);
    }

    count.to_string()
}

pub fn part2(input: String) -> String {
    let mut bricks = parse_bricks(&input).unwrap().1;
    bricks.sort();
    settle_bricks(&mut bricks);

    (0..bricks.len())
        .into_par_iter()
        .map(|i| count_falling_bricks(i, bricks.clone()))
        .sum::<usize>()
        .to_string()
}

fn settle_bricks(bricks: &mut Vec<Brick>) {
    for i in 0..bricks.len() {
        let mut brick = bricks.remove(i);
        let mut bottom_slice = brick.bottom_slice();
        while bottom_slice.min_corner.z > 0
            && bricks
                .iter()
                .take(i)
                .all(|brick| !brick.intersects(&bottom_slice))
        {
            brick.min_corner.z -= 1;
            brick.max_corner.z -= 1;
            bottom_slice.min_corner.z -= 1;
            bottom_slice.max_corner.z -= 1;
        }
        bricks.insert(i, brick);
    }
}

fn count_falling_bricks(index: usize, bricks: Vec<Brick>) -> usize {
    let mut count = 0;
    let mut bricks = bricks;

    bricks.remove(index);
    loop {
        let falling_bricks = bricks
            .iter()
            .filter(|brick| brick.can_fall(&bricks))
            .cloned()
            .collect::<Vec<_>>();

        if falling_bricks.is_empty() {
            break;
        }

        count += falling_bricks.len();

        bricks.retain(|brick| !falling_bricks.contains(brick));
    }

    count
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Brick {
    min_corner: Point3D<u16>,
    max_corner: Point3D<u16>,
}

impl Brick {
    fn can_fall(&self, bricks: &[Brick]) -> bool {
        if self.min_corner.z == 1 {
            return false;
        }
        let bottom_slice = self.bottom_slice();

        bricks
            .iter()
            .rev()
            .all(|brick| !bottom_slice.intersects(brick))
    }

    fn intersects(&self, other: &Self) -> bool {
        let x_overlap =
            self.min_corner.x <= other.max_corner.x && self.max_corner.x >= other.min_corner.x;
        let y_overlap =
            self.min_corner.y <= other.max_corner.y && self.max_corner.y >= other.min_corner.y;
        let z_overlap =
            self.min_corner.z <= other.max_corner.z && self.max_corner.z >= other.min_corner.z;

        x_overlap && y_overlap && z_overlap
    }

    fn bottom_slice(&self) -> Brick {
        Brick {
            min_corner: Point3D::new(self.min_corner.x, self.min_corner.y, self.min_corner.z - 1),
            max_corner: Point3D::new(self.max_corner.x, self.max_corner.y, self.min_corner.z - 1),
        }
    }
}

impl Ord for Brick {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.min_corner.z.cmp(&other.min_corner.z)
    }
}

impl PartialOrd for Brick {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_bricks(input: &str) -> IResult<&str, Vec<Brick>> {
    separated_list1(newline, parse_brick)(input)
}

fn parse_brick(input: &str) -> IResult<&str, Brick> {
    let (input, corner1) = parse_point(input)?;
    let (input, _) = tag("~")(input)?;
    let (input, corner2) = parse_point(input)?;

    Ok((
        input,
        Brick {
            min_corner: corner1,
            max_corner: corner2,
        },
    ))
}

fn parse_point(input: &str) -> IResult<&str, Point3D<u16>> {
    let (input, cords) = separated_list1(tag(","), nom::character::complete::digit1)(input)?;

    if cords.len() != 3 {
        panic!("Invalid point: {:?}", cords);
    }

    let x = cords[0].parse::<u16>().unwrap();
    let y = cords[1].parse::<u16>().unwrap();
    let z = cords[2].parse::<u16>().unwrap();

    Ok((input, Point3D::new(x, y, z)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_brick_intersects() {
        let brick1 = Brick {
            min_corner: Point3D::new(1, 1, 1),
            max_corner: Point3D::new(2, 2, 2),
        };

        let brick2 = Brick {
            min_corner: Point3D::new(2, 2, 2),
            max_corner: Point3D::new(3, 3, 3),
        };

        let brick3 = Brick {
            min_corner: Point3D::new(3, 3, 3),
            max_corner: Point3D::new(4, 4, 4),
        };

        assert!(brick1.intersects(&brick2));
        assert!(brick2.intersects(&brick1));

        assert!(!brick1.intersects(&brick3));
        assert!(!brick3.intersects(&brick1));
    }
}
