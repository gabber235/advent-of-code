use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug, Clone)]
pub struct Counter<T> {
    counts: HashMap<T, usize>,
}

impl<T: Eq + Hash> Counter<T> {
    pub fn new() -> Self {
        Self {
            counts: HashMap::new(),
        }
    }

    pub fn add(&mut self, item: T) {
        self.add_n(item, 1);
    }

    pub fn add_n(&mut self, item: T, n: usize) {
        *self.counts.entry(item).or_insert(0) += n;
    }

    pub fn subtract(&mut self, item: &T) -> bool {
        self.subtract_n(item, 1)
    }

    pub fn subtract_n(&mut self, item: &T, n: usize) -> bool {
        if let Some(count) = self.counts.get_mut(item) {
            if *count <= n {
                self.counts.remove(item);
            } else {
                *count -= n;
            }
            true
        } else {
            false
        }
    }

    pub fn get(&self, item: &T) -> usize {
        self.counts.get(item).copied().unwrap_or(0)
    }

    pub fn contains(&self, item: &T) -> bool {
        self.counts.contains_key(item)
    }

    pub fn remove(&mut self, item: &T) -> Option<usize> {
        self.counts.remove(item)
    }

    pub fn len(&self) -> usize {
        self.counts.len()
    }

    pub fn is_empty(&self) -> bool {
        self.counts.is_empty()
    }

    pub fn total(&self) -> usize {
        self.counts.values().sum()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&T, &usize)> {
        self.counts.iter()
    }

    pub fn keys(&self) -> impl Iterator<Item = &T> {
        self.counts.keys()
    }

    pub fn values(&self) -> impl Iterator<Item = &usize> {
        self.counts.values()
    }

    pub fn into_inner(self) -> HashMap<T, usize> {
        self.counts
    }

    pub fn as_inner(&self) -> &HashMap<T, usize> {
        &self.counts
    }
}

impl<T: Eq + Hash + Clone> Counter<T> {
    pub fn most_common(&self, n: usize) -> Vec<(T, usize)> {
        let mut items: Vec<_> = self.counts.iter().map(|(k, v)| (k.clone(), *v)).collect();
        items.sort_by(|a, b| b.1.cmp(&a.1));
        items.truncate(n);
        items
    }

    pub fn least_common(&self, n: usize) -> Vec<(T, usize)> {
        let mut items: Vec<_> = self.counts.iter().map(|(k, v)| (k.clone(), *v)).collect();
        items.sort_by(|a, b| a.1.cmp(&b.1));
        items.truncate(n);
        items
    }

    pub fn most_common_item(&self) -> Option<(T, usize)> {
        self.counts
            .iter()
            .max_by_key(|(_, v)| *v)
            .map(|(k, v)| (k.clone(), *v))
    }

    pub fn least_common_item(&self) -> Option<(T, usize)> {
        self.counts
            .iter()
            .min_by_key(|(_, v)| *v)
            .map(|(k, v)| (k.clone(), *v))
    }
}

impl<T: Eq + Hash> Default for Counter<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Eq + Hash> FromIterator<T> for Counter<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut counter = Counter::new();
        for item in iter {
            counter.add(item);
        }
        counter
    }
}

impl<T: Eq + Hash> Extend<T> for Counter<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for item in iter {
            self.add(item);
        }
    }
}

impl<T: Eq + Hash> IntoIterator for Counter<T> {
    type Item = (T, usize);
    type IntoIter = std::collections::hash_map::IntoIter<T, usize>;

    fn into_iter(self) -> Self::IntoIter {
        self.counts.into_iter()
    }
}

impl<'a, T: Eq + Hash> IntoIterator for &'a Counter<T> {
    type Item = (&'a T, &'a usize);
    type IntoIter = std::collections::hash_map::Iter<'a, T, usize>;

    fn into_iter(self) -> Self::IntoIter {
        self.counts.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_iter() {
        let counter: Counter<char> = "aabbbcccc".chars().collect();
        assert_eq!(counter.get(&'a'), 2);
        assert_eq!(counter.get(&'b'), 3);
        assert_eq!(counter.get(&'c'), 4);
        assert_eq!(counter.get(&'d'), 0);
    }

    #[test]
    fn test_add_and_subtract() {
        let mut counter = Counter::new();
        counter.add("apple");
        counter.add("apple");
        counter.add("banana");

        assert_eq!(counter.get(&"apple"), 2);
        assert_eq!(counter.get(&"banana"), 1);

        counter.subtract(&"apple");
        assert_eq!(counter.get(&"apple"), 1);

        counter.subtract(&"apple");
        assert_eq!(counter.get(&"apple"), 0);
        assert!(!counter.contains(&"apple"));
    }

    #[test]
    fn test_most_common() {
        let counter: Counter<char> = "aabbbcccc".chars().collect();
        let most = counter.most_common(2);
        assert_eq!(most[0], ('c', 4));
        assert_eq!(most[1], ('b', 3));
    }

    #[test]
    fn test_least_common() {
        let counter: Counter<char> = "aabbbcccc".chars().collect();
        let least = counter.least_common(2);
        assert_eq!(least[0], ('a', 2));
        assert_eq!(least[1], ('b', 3));
    }

    #[test]
    fn test_total() {
        let counter: Counter<char> = "aabbbcccc".chars().collect();
        assert_eq!(counter.total(), 9);
    }

    #[test]
    fn test_len() {
        let counter: Counter<char> = "aabbbcccc".chars().collect();
        assert_eq!(counter.len(), 3);
    }
}
