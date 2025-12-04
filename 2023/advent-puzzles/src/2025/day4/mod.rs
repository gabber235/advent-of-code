use crate::utils::{
    map::{GenerateMap, InteractWithPoint, IterAll},
    point::Point,
};
use array2d::Array2D;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Spot {
    Empty,
    Roll,
}

pub fn part1(input: String) -> String {
    let map = Array2D::generate_map(&input, |_, char| match char {
        '.' => Spot::Empty,
        '@' => Spot::Roll,
        _ => panic!("Invalid character"),
    })
    .expect("Failed to generate map");

    map.iter_all()
        .filter(|(point, spot)| **spot == Spot::Roll && is_accessable(&map, point))
        .count()
        .to_string()
}

fn is_accessable(map: &Array2D<Spot>, point: &Point) -> bool {
    map.eight_neighbours(point)
        .iter()
        .filter(
            |neighbour| match map.get_point(neighbour).expect("Invalid neighbour") {
                Spot::Empty => false,
                Spot::Roll => true,
            },
        )
        .count()
        < 4
}

pub fn part2(input: String) -> String {
    let mut map = Array2D::generate_map(&input, |_, char| match char {
        '.' => Spot::Empty,
        '@' => Spot::Roll,
        _ => panic!("Invalid character"),
    })
    .expect("Failed to generate map");

    let mut count = 0;
    loop {
        let removed = map
            .iter_all()
            .filter(|(point, spot)| **spot == Spot::Roll && is_accessable(&map, point))
            .map(|(point, _)| point)
            .collect::<Vec<_>>();

        if removed.is_empty() {
            break;
        }

        count += removed.len();
        for point in removed {
            map.set_point(&point, Spot::Empty)
                .expect("Failed to set point");
        }
    }

    count.to_string()
}
