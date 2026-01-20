use redis::{AsyncCommands, Client};
use serde::{de::DeserializeOwned, Serialize};
use std::time::Duration;

use crate::cache::CacheStore;
use crate::error::CacheError;

pub struct RedisCacheStore {
    client: Client,
}

impl RedisCacheStore {
    pub fn new(redis_url: &str) -> Result<Self, redis::RedisError> {
        let client = Client::open(redis_url)?;
        Ok(Self { client })
    }
}

#[async_trait::async_trait]
impl CacheStore for RedisCacheStore {
    async fn get<T>(&self, key: &str) -> Result<Option<T>, CacheError>
    where
        T: DeserializeOwned + Send,
    {
        let mut conn = self
            .client
            .get_async_connection()
            .await
            .map_err(CacheError::Redis)?;

        let raw: Option<String> = conn.get(key).await.map_err(CacheError::Redis)?;

        match raw {
            Some(json) => {
                let value = serde_json::from_str(&json).map_err(CacheError::Serialization)?;
                Ok(Some(value))
            }
            None => Ok(None),
        }
    }

    async fn set<T>(&self, key: &str, value: &T, ttl: Duration) -> Result<(), CacheError>
    where
        T: Serialize + Send + Sync,
    {
        let mut conn = self
            .client
            .get_async_connection()
            .await
            .map_err(CacheError::Redis)?;

        let json = serde_json::to_string(value).map_err(CacheError::Serialization)?;

        let _: () = conn
            .set_ex(key, json, ttl.as_secs())
            .await
            .map_err(CacheError::Redis)?;

        Ok(())
    }
}
