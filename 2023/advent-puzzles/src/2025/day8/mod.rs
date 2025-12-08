use rayon::prelude::*;

use crate::utils::point_3d::Point3D;

struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
            rank: vec![0; n],
            size: vec![1; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) -> bool {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x == root_y {
            return false;
        }

        if self.rank[root_x] < self.rank[root_y] {
            self.parent[root_x] = root_y;
            self.size[root_y] += self.size[root_x];
        } else if self.rank[root_x] > self.rank[root_y] {
            self.parent[root_y] = root_x;
            self.size[root_x] += self.size[root_y];
        } else {
            self.parent[root_y] = root_x;
            self.size[root_x] += self.size[root_y];
            self.rank[root_x] += 1;
        }
        true
    }
}

#[inline(always)]
fn parse_i64(s: &[u8]) -> i64 {
    let mut result: i64 = 0;
    for &b in s {
        result = result * 10 + (b - b'0') as i64;
    }
    result
}

fn parse_points(input: &str) -> Vec<Point3D<i64>> {
    input
        .lines()
        .map(|line| {
            let bytes = line.as_bytes();
            let mut start = 0;
            let mut idx = 0;

            while bytes[idx] != b',' {
                idx += 1;
            }
            let x = parse_i64(&bytes[start..idx]);

            idx += 1;
            start = idx;
            while bytes[idx] != b',' {
                idx += 1;
            }
            let y = parse_i64(&bytes[start..idx]);

            idx += 1;
            let z = parse_i64(&bytes[idx..]);

            Point3D { x, y, z }
        })
        .collect()
}

#[inline(always)]
fn distance_squared(a: &Point3D<i64>, b: &Point3D<i64>) -> i64 {
    let dx = a.x - b.x;
    let dy = a.y - b.y;
    let dz = a.z - b.z;
    dx * dx + dy * dy + dz * dz
}

#[inline(always)]
fn edge_index_to_pair(edge_idx: usize, n: usize) -> (usize, usize) {
    let k = edge_idx as f64;
    let nf = n as f64;
    let i = ((2.0 * nf - 1.0 - ((2.0 * nf - 1.0).powi(2) - 8.0 * k).sqrt()) / 2.0) as usize;
    let row_start = i * n - i * (i + 1) / 2;
    let j = i + 1 + (edge_idx - row_start);
    (i, j)
}

fn compute_edges(points: &[Point3D<i64>]) -> Vec<(usize, usize, i64)> {
    let n = points.len();
    let total_edges = n * (n - 1) / 2;

    (0..total_edges)
        .into_par_iter()
        .map(|edge_idx| {
            let (i, j) = edge_index_to_pair(edge_idx, n);
            let distance = distance_squared(&points[i], &points[j]);
            (i, j, distance)
        })
        .collect()
}

pub fn part1(input: String) -> String {
    let points = parse_points(&input);
    let n = points.len();
    let mut joins = compute_edges(&points);

    let num_connections = if n <= 20 { 10 } else { 1000 };

    if joins.len() > num_connections {
        joins.select_nth_unstable_by_key(num_connections, |&(_, _, d)| d);
        joins.truncate(num_connections);
        joins.sort_unstable_by_key(|&(_, _, d)| d);
    } else {
        joins.sort_unstable_by_key(|&(_, _, d)| d);
    }

    let mut uf = UnionFind::new(n);

    for &(i, j, _) in joins.iter().take(num_connections) {
        uf.union(i, j);
    }

    let mut seen_roots = vec![false; n];
    let mut sizes: Vec<usize> = Vec::new();

    for i in 0..n {
        let root = uf.find(i);
        if !seen_roots[root] {
            seen_roots[root] = true;
            sizes.push(uf.size[root]);
        }
    }

    if sizes.is_empty() {
        return "0".to_string();
    }

    sizes.sort_unstable_by(|a, b| b.cmp(a));
    let product: usize = sizes.iter().take(3).product();
    product.to_string()
}

pub fn part2(input: String) -> String {
    let points = parse_points(&input);
    let n = points.len();
    let mut joins = compute_edges(&points);

    joins.par_sort_unstable_by_key(|&(_, _, d)| d);

    let mut uf = UnionFind::new(n);
    let mut num_components = n;

    for &(i, j, _) in &joins {
        if uf.union(i, j) {
            num_components -= 1;
            if num_components == 1 {
                let product = points[i].x * points[j].x;
                return product.to_string();
            }
        }
    }

    panic!("No solution found");
}
