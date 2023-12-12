use std::borrow::Borrow;
use std::cell::Cell;
use std::fmt::{Debug, Formatter};
use std::hash::Hash;
use fxhash::{FxBuildHasher, FxHashMap};

#[derive(Debug, Clone)]
pub struct Cache<K, V> {
    // Record the value (V) and also the number of calculations required for it.
    // Elided calculations must count towards the hit ratio.
    inner: FxHashMap<K, (V, usize)>,
    stats: Cell<CacheStats>,
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct CacheStats {
    hits: usize,
    queries: usize,
}

impl Debug for CacheStats {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "hits = {}, queries = {}, ratio = {:.2}",
            self.hits, self.queries, self.hits as f32 / self.queries as f32
        )
    }
}

impl<K: Eq + Hash, V> Cache<K, V> {
    pub fn new() -> Self {
        Self {
            inner: FxHashMap::with_capacity_and_hasher(50_000, FxBuildHasher::default()),
            stats: Cell::new(CacheStats::default()),
        }
    }

    pub fn get<Q: ?Sized>(&self, key: &Q) -> Option<(&V, usize)> where K: Borrow<Q>, Q: Hash + Eq {
        let opt = self.inner.get(key);
        let mut stats = self.stats.get();

        // Record the number of hits and queries that *would* have happened,
        // had the search not been pruned
        let (hits, queries) = if let Some(&(_, calc_count)) = opt {
            (calc_count, calc_count)
        } else {
            (0, 1)
        };

        stats.hits += hits;
        stats.queries += queries;
        self.stats.set(stats);
        opt.map(|(value, calc_count)| (value, *calc_count))
    }

    pub fn insert(&mut self, key: K, value: V, calc_count: usize) {
        self.inner.insert(key, (value, calc_count));
    }

    #[allow(dead_code)]
    pub fn stats(&self) -> CacheStats {
        self.stats.get()
    }
}
