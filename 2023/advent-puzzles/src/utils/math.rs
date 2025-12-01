use std::ops::{Add, Div, Mul, Rem};

pub fn gcd<T>(a: T, b: T) -> T
where
    T: Copy + PartialEq + Rem<Output = T> + Default,
{
    let zero = T::default();
    if b == zero {
        a
    } else {
        gcd(b, a % b)
    }
}

pub fn lcm<T>(a: T, b: T) -> T
where
    T: Copy + PartialEq + Rem<Output = T> + Mul<Output = T> + Div<Output = T> + Default,
{
    let zero = T::default();
    if a == zero || b == zero {
        zero
    } else {
        a / gcd(a, b) * b
    }
}

pub fn lcm_many<T, I>(iter: I) -> T
where
    T: Copy + PartialEq + Rem<Output = T> + Mul<Output = T> + Div<Output = T> + Default,
    I: IntoIterator<Item = T>,
{
    iter.into_iter().fold(T::default(), |acc, x| {
        if acc == T::default() {
            x
        } else {
            lcm(acc, x)
        }
    })
}

pub fn gcd_many<T, I>(iter: I) -> T
where
    T: Copy + PartialEq + Rem<Output = T> + Default,
    I: IntoIterator<Item = T>,
{
    iter.into_iter().reduce(gcd).unwrap_or_else(T::default)
}

pub fn mod_positive(n: i64, m: i64) -> i64 {
    ((n % m) + m) % m
}

pub fn mod_positive_i32(n: i32, m: i32) -> i32 {
    ((n % m) + m) % m
}

pub fn sum<T, I>(iter: I) -> T
where
    T: Add<Output = T> + Default,
    I: IntoIterator<Item = T>,
{
    iter.into_iter().fold(T::default(), |acc, x| acc + x)
}

pub fn product<T, I>(iter: I) -> T
where
    T: Mul<Output = T> + From<u8>,
    I: IntoIterator<Item = T>,
{
    iter.into_iter().fold(T::from(1u8), |acc, x| acc * x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(12, 8), 4);
        assert_eq!(gcd(17, 13), 1);
        assert_eq!(gcd(100, 25), 25);
        assert_eq!(gcd(0, 5), 5);
        assert_eq!(gcd(5, 0), 5);
    }

    #[test]
    fn test_lcm() {
        assert_eq!(lcm(4, 6), 12);
        assert_eq!(lcm(3, 5), 15);
        assert_eq!(lcm(12, 18), 36);
    }

    #[test]
    fn test_lcm_many() {
        assert_eq!(lcm_many([4, 6, 8]), 24);
        assert_eq!(lcm_many([2, 3, 5, 7]), 210);
    }

    #[test]
    fn test_gcd_many() {
        assert_eq!(gcd_many([12, 18, 24]), 6);
        assert_eq!(gcd_many([100, 50, 25]), 25);
    }

    #[test]
    fn test_mod_positive() {
        assert_eq!(mod_positive(7, 3), 1);
        assert_eq!(mod_positive(-1, 3), 2);
        assert_eq!(mod_positive(-7, 3), 2);
        assert_eq!(mod_positive(0, 5), 0);
    }

    #[test]
    fn test_sum() {
        assert_eq!(sum([1, 2, 3, 4, 5]), 15);
        assert_eq!(sum(Vec::<i32>::new()), 0);
    }

    #[test]
    fn test_product() {
        assert_eq!(product([1, 2, 3, 4]), 24);
        assert_eq!(product([5, 5]), 25);
    }
}
