use std::collections::BTreeMap;

use crate::utils::point_3d::Point3D;

pub fn part1(input: String) -> String {
    let points = input
        .lines()
        .map(|line| {
            let cords = line
                .split(',')
                .map(|num| num.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();
            Point3D {
                x: cords[0],
                y: cords[1],
                z: cords[2],
            }
        })
        .collect::<Vec<Point3D<i64>>>();

    let mut joins: Vec<(usize, usize, i64)> = vec![];

    for (i, point) in points.iter().enumerate() {
        for (j, other) in points.iter().enumerate().skip(i) {
            if i != j {
                let distance = point.distanceSquared(other);
                joins.push((i, j, distance));
            }
        }
    }

    joins.sort_by(|a, b| a.2.cmp(&b.2));

    let mut circuits = BTreeMap::<usize, Circuit>::new();

    let num_connections = if points.len() <= 20 { 10 } else { 1000 };

    for (i, j, _) in joins.iter().take(num_connections) {
        let circuit_i_key = circuits
            .iter()
            .find(|(_, c)| c.contains(*i))
            .map(|(k, _)| *k);
        let circuit_j_key = circuits
            .iter()
            .find(|(_, c)| c.contains(*j))
            .map(|(k, _)| *k);

        match (circuit_i_key, circuit_j_key) {
            (Some(k1), Some(k2)) => {
                if k1 != k2 {
                    let c2 = circuits.remove(&k2).unwrap();
                    circuits.get_mut(&k1).unwrap().merge(c2);
                }
            }
            (Some(k), None) => {
                circuits.get_mut(&k).unwrap().add(*j);
            }
            (None, Some(k)) => {
                circuits.get_mut(&k).unwrap().add(*i);
            }
            (None, None) => {
                let mut new_circuit = Circuit::new();
                new_circuit.add(*i);
                new_circuit.add(*j);
                circuits.insert(*i, new_circuit);
            }
        }
    }

    let mut sizes: Vec<usize> = circuits.values().map(|c| c.points.len()).collect();
    if sizes.is_empty() {
        return "0".to_string();
    }
    sizes.sort_unstable_by(|a, b| b.cmp(a));
    let product: usize = sizes.iter().take(3).product();
    return product.to_string();
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Circuit {
    points: Vec<usize>,
}

impl Circuit {
    fn new() -> Self {
        Circuit { points: vec![] }
    }

    fn add(&mut self, point: usize) {
        self.points.push(point);
    }

    fn merge(&mut self, other: Circuit) {
        self.points.extend(other.points);
    }

    fn contains(&self, point: usize) -> bool {
        self.points.contains(&point)
    }
}

pub fn part2(input: String) -> String {
    let points = input
        .lines()
        .map(|line| {
            let cords = line
                .split(',')
                .map(|num| num.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();
            Point3D {
                x: cords[0],
                y: cords[1],
                z: cords[2],
            }
        })
        .collect::<Vec<Point3D<i64>>>();

    let mut joins: Vec<(usize, usize, i64)> = vec![];

    for (i, point) in points.iter().enumerate() {
        for (j, other) in points.iter().enumerate().skip(i) {
            if i != j {
                let distance = point.distanceSquared(other);
                joins.push((i, j, distance));
            }
        }
    }

    joins.sort_by(|a, b| a.2.cmp(&b.2));

    let mut circuits = BTreeMap::<usize, Circuit>::new();

    for (i, j, _) in joins.iter() {
        let circuit_i_key = circuits
            .iter()
            .find(|(_, c)| c.contains(*i))
            .map(|(k, _)| *k);
        let circuit_j_key = circuits
            .iter()
            .find(|(_, c)| c.contains(*j))
            .map(|(k, _)| *k);

        match (circuit_i_key, circuit_j_key) {
            (Some(k1), Some(k2)) => {
                if k1 != k2 {
                    let c2 = circuits.remove(&k2).unwrap();
                    circuits.get_mut(&k1).unwrap().merge(c2);
                }
            }
            (Some(k), None) => {
                circuits.get_mut(&k).unwrap().add(*j);
            }
            (None, Some(k)) => {
                circuits.get_mut(&k).unwrap().add(*i);
            }
            (None, None) => {
                let mut new_circuit = Circuit::new();
                new_circuit.add(*i);
                new_circuit.add(*j);
                circuits.insert(*i, new_circuit);
            }
        }

        if circuits.len() == 1 && circuits.first_key_value().unwrap().1.points.len() == points.len()
        {
            let point_i = points[*i];
            let point_j = points[*j];
            let product = point_i.x * point_j.x;
            return product.to_string();
        }
    }

    panic!("No solution found");
}
