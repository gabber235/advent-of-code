use i_overlay::{
    core::{fill_rule::FillRule, overlay_rule::OverlayRule},
    float::single::SingleFloatOverlay,
    i_shape::float::area::Area,
};
use itertools::Itertools;
use rayon::iter::{ParallelBridge, ParallelIterator};

use crate::utils::point::Point;

pub fn part1(input: String) -> String {
    let points = parse_points(&input);

    points
        .iter()
        .tuple_combinations()
        .par_bridge()
        .map(|(a, b)| a.area_to(b))
        .max()
        .unwrap_or(0)
        .to_string()
}

fn parse_points(input: &str) -> Vec<Point> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(',');
            let x = parts.next().unwrap().parse::<i32>().unwrap();
            let y = parts.next().unwrap().parse::<i32>().unwrap();
            Point::new(x, y)
        })
        .collect()
}

impl Point {
    pub fn area_to(&self, other: &Point) -> usize {
        let width = (other.x - self.x).abs() as usize + 1;
        let height = (other.y - self.y).abs() as usize + 1;
        width * height
    }
}

pub fn part2(input: String) -> String {
    let points = parse_points(&input);

    points
        .iter()
        .tuple_combinations()
        .par_bridge()
        .filter(|(a, b)| {
            let min_x = a.x.min(b.x);
            let max_x = a.x.max(b.x);
            let min_y = a.y.min(b.y);
            let max_y = a.y.max(b.y);
            let rect = [
                Point::new(min_x, min_y),
                Point::new(min_x, max_y),
                Point::new(max_x, max_y),
                Point::new(max_x, min_y),
            ];

            let shape = rect.overlay(&points, OverlayRule::Difference, FillRule::EvenOdd);
            shape.area() == 0f32
        })
        .map(|(a, b)| a.area_to(b))
        .max()
        .unwrap_or(0)
        .to_string()
}
