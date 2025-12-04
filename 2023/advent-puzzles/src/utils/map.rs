use array2d::Array2D;
use std::fmt::Write;

use super::point::Point;

pub trait GenerateMap {
    type Item;
    fn generate_map<F: FnMut(Point, char) -> Self::Item>(
        input: &str,
        f: F,
    ) -> Result<Self, array2d::Error>
    where
        Self: Sized;
}

impl<T> GenerateMap for Array2D<T>
where
    T: Clone + Sized,
{
    type Item = T;

    fn generate_map<F: FnMut(Point, char) -> Self::Item>(
        input: &str,
        mut f: F,
    ) -> Result<Self, array2d::Error> {
        let vec: Vec<Vec<Self::Item>> = input
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, c)| f(Point::new(x as i32, y as i32), c))
                    .collect()
            })
            .collect();

        Array2D::from_rows(&vec)
    }
}

pub trait InteractWithPoint {
    type Item;
    fn get_point(&self, point: &Point) -> Option<&Self::Item>;
    fn get_point_mut(&mut self, point: &Point) -> Option<&mut Self::Item>;
    fn get_looping_point(&self, point: &Point) -> &Self::Item;
    fn set_point(&mut self, point: &Point, item: Self::Item) -> Result<(), array2d::Error>;

    /// Returns the four orthogonal neighbours (up, down, left, right) of `point`
    /// that are inside the bounds of the map.
    fn four_neighbours(&self, point: &Point) -> Vec<Point>;

    /// Returns the eight neighbours (including diagonals) of `point`
    /// that are inside the bounds of the map.
    fn eight_neighbours(&self, point: &Point) -> Vec<Point>;
}

impl<T> InteractWithPoint for Array2D<T> {
    type Item = T;

    fn get_point(&self, point: &Point) -> Option<&Self::Item> {
        let point = point.ok_map(self)?;
        self.get(point.y as usize, point.x as usize)
    }

    fn get_point_mut(&mut self, point: &Point) -> Option<&mut Self::Item> {
        let point = point.ok_map(self)?;
        self.get_mut(point.y as usize, point.x as usize)
    }

    fn get_looping_point(&self, point: &Point) -> &Self::Item {
        let point = point.looping_map(self);
        self.get_point(&point).expect("Point should be valid")
    }

    fn set_point(&mut self, point: &Point, item: Self::Item) -> Result<(), array2d::Error> {
        let point = point.ok_map(self).ok_or(array2d::Error::IndexOutOfBounds(
            point.index(self.num_rows()),
        ))?;
        self.set(point.y as usize, point.x as usize, item)
    }

    fn four_neighbours(&self, point: &Point) -> Vec<Point> {
        // Offsets for up, down, left, right
        let offsets = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        let mut neighbours = Vec::with_capacity(4);
        if let Some(base) = point.ok_map(self) {
            for (dx, dy) in offsets.iter() {
                let candidate = Point::new(base.x + dx, base.y + dy);
                if candidate.ok_map(self).is_some() {
                    neighbours.push(candidate);
                }
            }
        }
        neighbours
    }

    fn eight_neighbours(&self, point: &Point) -> Vec<Point> {
        // Offsets for the eight surrounding cells
        let offsets = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];
        let mut neighbours = Vec::with_capacity(8);
        if let Some(base) = point.ok_map(self) {
            for (dx, dy) in offsets.iter() {
                let candidate = Point::new(base.x + dx, base.y + dy);
                if candidate.ok_map(self).is_some() {
                    neighbours.push(candidate);
                }
            }
        }
        neighbours
    }
}

pub trait IterAll {
    type Item;
    fn iter_all(&self) -> MapIter<'_, Self::Item>;
}

impl<T> IterAll for Array2D<T>
where
    T: Clone + Sized,
{
    type Item = T;

    fn iter_all(&self) -> MapIter<'_, Self::Item> {
        MapIter {
            map: self,
            index: 0,
        }
    }
}

pub struct MapIter<'a, T> {
    map: &'a Array2D<T>,
    index: usize,
}

impl<'a, T> Iterator for MapIter<'a, T>
where
    T: Clone + Sized,
{
    type Item = (Point, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        let point = Point::from_index(self.index, self.map.num_columns());
        self.index += 1;
        self.map.get_point(&point).map(|item| (point, item))
    }
}

pub trait PrintMap {
    fn print_map(&self);
}

impl<T: std::fmt::Display> PrintMap for Array2D<T> {
    fn print_map(&self) {
        for row in self.rows_iter() {
            for item in row {
                print!("{}", item);
            }
            println!();
        }
    }
}
pub trait PrintMapWith {
    type Item;
    fn print_with<F: Fn(&Point, &Self::Item, &mut String)>(&self, f: F)
    where
        Self: Sized;
}

impl<T> PrintMapWith for Array2D<T>
where
    T: Clone + Sized,
{
    type Item = T;

    fn print_with<F: Fn(&Point, &Self::Item, &mut String)>(&self, f: F) {
        let mut string = String::new();
        self.iter_all().for_each(|(point, item)| {
            f(&point, item, &mut string);
            if point.x == self.num_columns() as i32 - 1 {
                writeln!(string).unwrap();
            }
        });
        print!("{}", string);
    }
}
