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
}

pub trait IterAll {
    type Item;
    fn iter_all(&self) -> MapIter<Self::Item>;
}

impl<T> IterAll for Array2D<T>
where
    T: Clone + Sized,
{
    type Item = T;

    fn iter_all(&self) -> MapIter<Self::Item> {
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
