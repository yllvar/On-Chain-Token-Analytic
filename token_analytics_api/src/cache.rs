use redis::{AsyncCommands, Client};
use anyhow::Result;

pub type RedisPool = Client;

pub async fn init_redis_client(url: &str) -> Result<RedisPool> {
    let client = Client::open(url)?;
    
    // Test connection
    let mut conn = client.get_async_connection().await?;
    let _: String = redis::cmd("PING").query_async(&mut conn).await?;
    
    Ok(client)
}

pub async fn cache_set(
    client: &RedisPool,
    key: &str,
    value: &str,
    ttl_seconds: u64,
) -> Result<()> {
    let mut conn = client.get_async_connection().await?;
    if ttl_seconds > 0 {
        conn.set_ex::<_, _, ()>(key, value, ttl_seconds).await?;
    } else {
        conn.set::<_, _, ()>(key, value).await?;
    }
    Ok(())
}

pub async fn cache_get(client: &RedisPool, key: &str) -> Result<Option<String>> {
    let mut conn = client.get_async_connection().await?;
    let value: Option<String> = conn.get(key).await?;
    Ok(value)
}

pub async fn invalidate_cache(client: &RedisPool, pattern: &str) -> Result<()> {
    let mut conn = client.get_async_connection().await?;
    let keys: Vec<String> = conn.keys(pattern).await?;
    
    if !keys.is_empty() {
        // Convert Vec<String> to Vec<&str> for redis command
        let key_refs: Vec<&str> = keys.iter().map(|s| s.as_str()).collect();
        let _: i64 = redis::cmd("DEL")
            .arg(key_refs)
            .query_async(&mut conn)
            .await?;
    }
    
    Ok(())
}
