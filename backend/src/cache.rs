use std::time::{Duration, Instant};
use std::sync::RwLock;

use crate::models::AnalyzedMarket;

pub trait CacheStore: Send + Sync {
    fn get_active(&self) -> Option<Vec<AnalyzedMarket>>;
    fn get_all(&self) -> Option<Vec<AnalyzedMarket>>;
    fn set_active(&self, markets: Vec<AnalyzedMarket>);
    fn set_all(&self, markets: Vec<AnalyzedMarket>);
}

struct Cached<T> {
    value: T,
    expires_at: Instant,
}

pub struct InMemoryCacheStore {
    ttl: Duration,
    active: RwLock<Option<Cached<Vec<AnalyzedMarket>>>>,
    all: RwLock<Option<Cached<Vec<AnalyzedMarket>>>>,
}

impl InMemoryCacheStore {
    pub fn new(ttl: Duration) -> Self {
        Self {
            ttl,
            active: RwLock::new(None),
            all: RwLock::new(None),
        }
    }

    fn is_valid<T>(cached: &Cached<T>) -> bool {
        Instant::now() < cached.expires_at
    }
}

impl CacheStore for InMemoryCacheStore {
    fn get_active(&self) -> Option<Vec<AnalyzedMarket>> {
        let guard = self.active.read().ok()?;
        guard
            .as_ref()
            .filter(|c| Self::is_valid(c))
            .map(|c| c.value.clone())
    }

    fn get_all(&self) -> Option<Vec<AnalyzedMarket>> {
        let guard = self.all.read().ok()?;
        guard
            .as_ref()
            .filter(|c| Self::is_valid(c))
            .map(|c| c.value.clone())
    }

    fn set_active(&self, markets: Vec<AnalyzedMarket>) {
        let mut guard = self.active.write().unwrap();
        *guard = Some(Cached {
            value: markets,
            expires_at: Instant::now() + self.ttl,
        });
    }

    fn set_all(&self, markets: Vec<AnalyzedMarket>) {
        let mut guard = self.all.write().unwrap();
        *guard = Some(Cached {
            value: markets,
            expires_at: Instant::now() + self.ttl,
        });
    }
}
