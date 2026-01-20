use std::collections::HashMap;
use std::sync::RwLock;
use std::time::{Duration, Instant};

use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};

use crate::error::CacheError;

pub mod redis;

#[async_trait]
pub trait CacheStore: Send + Sync {
    async fn get<T>(&self, key: &str) -> Result<Option<T>, CacheError>
    where
        T: DeserializeOwned + Send;

    async fn set<T>(&self, key: &str, value: &T, ttl: Duration) -> Result<(), CacheError>
    where
        T: Serialize + Send + Sync;
}

struct Cached {
    value: String,
    expires_at: Instant,
}

pub struct InMemoryCacheStore {
    entries: RwLock<HashMap<String, Cached>>,
}

impl InMemoryCacheStore {
    pub fn new() -> Self {
        Self {
            entries: RwLock::new(HashMap::new()),
        }
    }

    fn is_valid(cached: &Cached) -> bool {
        Instant::now() < cached.expires_at
    }
}

pub enum CacheStoreImpl {
    InMemory(InMemoryCacheStore),
    Redis(redis::RedisCacheStore),
}

#[async_trait]
impl CacheStore for InMemoryCacheStore {
    async fn get<T>(&self, key: &str) -> Result<Option<T>, CacheError>
    where
        T: DeserializeOwned + Send,
    {
        let guard = self
            .entries
            .read()
            .map_err(|_| CacheError::Internal("cache lock poisoned".to_string()))?;

        match guard.get(key) {
            Some(cached) if Self::is_valid(cached) => {
                // JSON round-trip keeps cache backend neutral.
                let value =
                    serde_json::from_str(&cached.value).map_err(CacheError::Serialization)?;
                Ok(Some(value))
            }
            _ => Ok(None),
        }
    }

    async fn set<T>(&self, key: &str, value: &T, ttl: Duration) -> Result<(), CacheError>
    where
        T: Serialize + Send + Sync,
    {
        let mut guard = self
            .entries
            .write()
            .map_err(|_| CacheError::Internal("cache lock poisoned".to_string()))?;

        let json = serde_json::to_string(value).map_err(CacheError::Serialization)?;
        guard.insert(
            key.to_string(),
            Cached {
                value: json,
                expires_at: Instant::now() + ttl,
            },
        );
        Ok(())
    }
}

#[async_trait]
impl CacheStore for CacheStoreImpl {
    async fn get<T>(&self, key: &str) -> Result<Option<T>, CacheError>
    where
        T: DeserializeOwned + Send,
    {
        match self {
            CacheStoreImpl::InMemory(inner) => inner.get(key).await,
            CacheStoreImpl::Redis(inner) => inner.get(key).await,
        }
    }

    async fn set<T>(&self, key: &str, value: &T, ttl: Duration) -> Result<(), CacheError>
    where
        T: Serialize + Send + Sync,
    {
        match self {
            CacheStoreImpl::InMemory(inner) => inner.set(key, value, ttl).await,
            CacheStoreImpl::Redis(inner) => inner.set(key, value, ttl).await,
        }
    }
}
