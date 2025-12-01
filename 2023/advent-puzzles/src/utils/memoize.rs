use std::sync::atomic::{AtomicU64, Ordering};

#[derive(Debug, Clone)]
pub struct MemoizeStats {
    pub function_name: &'static str,
    pub hits: u64,
    pub misses: u64,
    pub cache_size: usize,
}

impl MemoizeStats {
    pub fn hit_rate(&self) -> f64 {
        let total = self.hits + self.misses;
        if total == 0 {
            0.0
        } else {
            (self.hits as f64 / total as f64) * 100.0
        }
    }

    pub fn total_calls(&self) -> u64 {
        self.hits + self.misses
    }
}

pub struct MemoizeStatsProvider {
    pub name: &'static str,
    pub get_stats: fn() -> MemoizeStats,
    pub clear_cache: fn(),
    pub reset_stats: fn(),
}

inventory::collect!(MemoizeStatsProvider);

pub fn clear_all_caches_and_stats() {
    for provider in inventory::iter::<MemoizeStatsProvider> {
        (provider.clear_cache)();
        (provider.reset_stats)();
    }
}

pub fn collect_used_stats() -> Vec<MemoizeStats> {
    inventory::iter::<MemoizeStatsProvider>
        .into_iter()
        .map(|provider| (provider.get_stats)())
        .filter(|stats| stats.hits + stats.misses > 0)
        .collect()
}

pub fn print_memoize_stats() {
    let stats = collect_used_stats();
    if stats.is_empty() {
        return;
    }

    use colored::Colorize;

    println!("\n");
    println!("{}", "📊 Memoization Statistics".cyan().bold());

    for stat in stats {
        println!(
            "   {}: {} hits │ {} misses │ {} hit rate │ {} cached",
            stat.function_name.bold(),
            format_number(stat.hits).green(),
            format_number(stat.misses).yellow(),
            format!("{:.1}%", stat.hit_rate()).blue(),
            format_number(stat.cache_size as u64).dimmed()
        );
    }
    println!("{}", "─".repeat(60).dimmed());
}

fn format_number(n: u64) -> String {
    let s = n.to_string();
    let mut result = String::new();
    for (i, c) in s.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            result.insert(0, ',');
        }
        result.insert(0, c);
    }
    result
}

pub struct AtomicStats {
    pub hits: AtomicU64,
    pub misses: AtomicU64,
}

impl AtomicStats {
    pub const fn new() -> Self {
        Self {
            hits: AtomicU64::new(0),
            misses: AtomicU64::new(0),
        }
    }

    pub fn record_hit(&self) {
        self.hits.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_miss(&self) {
        self.misses.fetch_add(1, Ordering::Relaxed);
    }

    pub fn get_hits(&self) -> u64 {
        self.hits.load(Ordering::Relaxed)
    }

    pub fn get_misses(&self) -> u64 {
        self.misses.load(Ordering::Relaxed)
    }

    pub fn reset(&self) {
        self.hits.store(0, Ordering::Relaxed);
        self.misses.store(0, Ordering::Relaxed);
    }
}
