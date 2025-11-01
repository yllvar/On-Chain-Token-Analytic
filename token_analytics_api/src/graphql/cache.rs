use async_graphql::Result;
use serde_json;
use crate::models::{Token, TokenAnalytics};
use crate::cache::{cache_get, cache_set, RedisPool};

const CACHE_TTL: u64 = 300; // 5 minutes

pub async fn get_token(redis: &RedisPool, key: &str) -> Result<Option<Token>> {
    if let Some(cached) = cache_get(redis, key).await? {
        Ok(serde_json::from_str(&cached)?)
    } else {
        Ok(None)
    }
}

pub async fn cache_token(redis: &RedisPool, key: &str, token: &Token) -> Result<()> {
    let serialized = serde_json::to_string(token)?;
    cache_set(redis, key, &serialized, CACHE_TTL).await?;
    Ok(())
}

pub async fn get_analytics(redis: &RedisPool, key: &str) -> Result<Option<TokenAnalytics>> {
    if let Some(cached) = cache_get(redis, key).await? {
        Ok(serde_json::from_str(&cached)?)
    } else {
        Ok(None)
    }
}

pub async fn cache_analytics(redis: &RedisPool, key: &str, analytics: &TokenAnalytics) -> Result<()> {
    let serialized = serde_json::to_string(analytics)?;
    cache_set(redis, key, &serialized, CACHE_TTL).await?;
    Ok(())
}

pub async fn invalidate(redis: &RedisPool, key: &str) -> Result<()> {
    // Invalidate by setting a very short TTL
    cache_set(redis, key, "", 1).await?;
    Ok(())
}
