use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};
use std::time::Duration;

use backend::cache::{CacheStore, InMemoryCacheStore};

#[tokio::test]
async fn cache_hit_before_ttl_expiry() {
    let cache = InMemoryCacheStore::new();
    let ttl = Duration::from_millis(50);

    cache.set("key", &42u32, ttl).await.expect("set failed");
    let value = cache.get::<u32>("key").await.expect("get failed");

    assert_eq!(value, Some(42u32));
}

#[tokio::test]
async fn cache_miss_after_ttl_expiry() {
    let cache = InMemoryCacheStore::new();
    let ttl = Duration::from_millis(20);

    cache.set("key", &7u32, ttl).await.expect("set failed");
    tokio::time::sleep(Duration::from_millis(30)).await;

    let value = cache.get::<u32>("key").await.expect("get failed");
    assert_eq!(value, None);
}

#[tokio::test]
async fn read_through_only_fetches_once_per_ttl() {
    let cache = InMemoryCacheStore::new();
    let ttl = Duration::from_millis(40);
    let fetches = Arc::new(AtomicUsize::new(0));

    async fn read_through(cache: &InMemoryCacheStore, ttl: Duration, fetches: &AtomicUsize) -> u32 {
        if let Some(value) = cache.get::<u32>("key").await.unwrap() {
            return value;
        }

        fetches.fetch_add(1, Ordering::SeqCst);
        let value = 99u32;
        cache.set("key", &value, ttl).await.unwrap();
        value
    }

    read_through(&cache, ttl, &fetches).await;
    read_through(&cache, ttl, &fetches).await;

    assert_eq!(fetches.load(Ordering::SeqCst), 1);

    tokio::time::sleep(Duration::from_millis(50)).await;
    read_through(&cache, ttl, &fetches).await;

    assert_eq!(fetches.load(Ordering::SeqCst), 2);
}
