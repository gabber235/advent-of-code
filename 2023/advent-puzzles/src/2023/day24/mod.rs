use std::{f64, fmt::Display};

use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{i64, newline, space0},
    multi::separated_list1,
    sequence::tuple,
    IResult,
};
use rayon::iter::ParallelBridge;
use rayon::prelude::*;

use crate::utils::point_3d::Point3D;

pub fn part1(input: String) -> String {
    let hails = parse(&input).unwrap().1;

    let range = 200000000000000..=400000000000000;

    // hails.iter().tuple_combinations().for_each(|(a, b)| {
    //     if let Some(intersection) = a.intersection_2d(b) {
    //         println!("{} & {} -> {}", a, b, intersection);
    //     } else {
    //         println!("{} & {} -> nothing", a, b);
    //     }
    // });
    hails
        .iter()
        .tuple_combinations()
        .par_bridge()
        .flat_map(|(a, b)| a.intersection_2d(b))
        .filter(|p| range.contains(&(p.x as i64)) && range.contains(&(p.y as i64)))
        .count()
        .to_string()
}

pub fn part2(input: String) -> String {
    let hails = parse(&input).unwrap().1;

    // Write a z3 formula to solve this problem and write it to a file
    // We want to solve for x,y,z,vx,vy,vz.
    // x + vx * t == a + va * t
    // y + vy * t == b + vb * t
    // z + vz * t == c + vc * t
    // Where a,b,c,va,vb,vc are the position and velocity of the hails

    let mut formula = String::new();

    // Add the variables
    formula.push_str("(declare-const x Real)\n");
    formula.push_str("(declare-const y Real)\n");
    formula.push_str("(declare-const z Real)\n");
    formula.push_str("(declare-const vx Real)\n");
    formula.push_str("(declare-const vy Real)\n");
    formula.push_str("(declare-const vz Real)\n");

    // Add the constraints for each hailstone
    for (i, hail) in hails.iter().enumerate().take(4) {
        let Point3D { x: a, y: b, z: c } = hail.position;
        let Point3D {
            x: va,
            y: vb,
            z: vc,
        } = hail.velocity;

        let t = format!("t{}", i); // Unique time variable for each hailstone
        formula.push_str(&format!("(declare-const {} Real)\n", t));
        formula.push_str(&format!(
            "(assert (= (+ x (* vx {})) (+ {} (* {} {}))))\n",
            t, a, va, t
        ));
        formula.push_str(&format!(
            "(assert (= (+ y (* vy {})) (+ {} (* {} {}))))\n",
            t, b, vb, t
        ));
        formula.push_str(&format!(
            "(assert (= (+ z (* vz {})) (+ {} (* {} {}))))\n",
            t, c, vc, t
        ));
    }

    // Add the solver and the output
    formula.push_str("(check-sat)\n");
    formula.push_str("(get-model)\n");

    // Write the formula to a file
    std::fs::write("formula.smt2", formula).unwrap();

    // Run the solver
    let output = std::process::Command::new("z3")
        .arg("formula.smt2")
        .output()
        .unwrap();

    // Parse the output
    let output = String::from_utf8(output.stdout).unwrap();

    output
    //     .lines()
    //     .filter(|l| l.starts_with("x"))
    //     .map(|l| l.split_whitespace().nth(1).unwrap().parse::<f64>().unwrap())
    //     .sum::<f64>()
    //     .to_string()
}

#[derive(Debug, PartialEq)]
struct Hail {
    position: Point3D<f64>,
    velocity: Point3D<f64>,
}

impl Hail {
    fn intersection_2d(&self, other: &Hail) -> Option<Point3D<f64>> {
        let det = self.velocity.x * other.velocity.y - other.velocity.x * self.velocity.y;

        // If the determinant is zero, the lines are parallel and there is no intersection.
        if det.abs() <= f64::EPSILON {
            return None;
        }

        // Use Cramer's Rule to solve for 't' and 'u'
        let t = (other.velocity.x * (self.position.y - other.position.y)
            - other.velocity.y * (self.position.x - other.position.x))
            / det;
        let u = (self.velocity.x * (self.position.y - other.position.y)
            - self.velocity.y * (self.position.x - other.position.x))
            / det;

        // If 't' or 'u' are < 0 then they intersected behind the origin
        if t < 0.0 || u < 0.0 {
            return None;
        }

        // Get the point of intersection
        let intersection = Point3D::new(
            self.position.x + t * self.velocity.x,
            self.position.y + t * self.velocity.y,
            0.0,
        );
        Some(intersection)
    }
}

impl Display for Hail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} @ {}", self.position, self.velocity)
    }
}

fn parse(input: &str) -> IResult<&str, Vec<Hail>> {
    separated_list1(newline, parse_hail)(input)
}

fn parse_hail(input: &str) -> IResult<&str, Hail> {
    let (input, position) = parse_point(input)?;
    let (input, _) = space0(input)?;
    let (input, _) = tag("@")(input)?;
    let (input, _) = space0(input)?;
    let (input, velocity) = parse_point(input)?;

    Ok((input, Hail { position, velocity }))
}

fn parse_point(input: &str) -> IResult<&str, Point3D<f64>> {
    let (input, cords) = tuple((i64, parse_seperator, i64, parse_seperator, i64))(input)?;

    Ok((
        input,
        Point3D::new(cords.0 as f64, cords.2 as f64, cords.4 as f64),
    ))
}

fn parse_seperator(input: &str) -> IResult<&str, &str> {
    let (input, _) = tag(",")(input)?;
    let (input, _) = space0(input)?;

    Ok((input, ","))
}
