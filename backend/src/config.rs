use std::time::Duration;

pub struct CacheConfig {
    pub backend: CacheBackend,
    pub ttl: Duration,
}

pub enum CacheBackend {
    InMemory,
    Redis { url: String },
}

impl CacheConfig {
    pub fn from_env() -> Self {
        let ttl = std::env::var("CACHE_TTL_SECONDS")
            .ok()
            .and_then(|v| v.parse::<u64>().ok())
            .unwrap_or(30);

        match std::env::var("REDIS_URL") {
            Ok(url) => CacheConfig {
                backend: CacheBackend::Redis { url },
                ttl: Duration::from_secs(ttl),
            },
            Err(_) => CacheConfig {
                backend: CacheBackend::InMemory,
                ttl: Duration::from_secs(ttl),
            },
        }
    }
}
