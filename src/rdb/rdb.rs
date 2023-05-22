//! redis 操作

use redis::{aio::Connection, AsyncCommands, Client};
use serde::{de::DeserializeOwned, Serialize};

use crate::{Error, Result};

/// 获取连接
async fn get_conn(client: &Client) -> Result<Connection> {
    client.get_async_connection().await.map_err(Error::from)
}

/// 将数据写入 redis
pub async fn set_ex(client: &Client, key: &str, value: &str, sec: usize) -> Result<()> {
    let mut conn = get_conn(client).await?;
    conn.set_ex(key, value, sec).await.map_err(Error::from)
}

pub async fn set(client: &Client, key: &str, value: &str) -> Result<()> {
    let mut conn = get_conn(client).await?;
    conn.set(key, value).await.map_err(Error::from)
}
pub async fn set_json<T: Serialize>(client: &Client, key: &str, value: &T) -> Result<()> {
    let v = serde_json::to_string(value).map_err(Error::from)?;
    set(client, key, &v).await
}
pub async fn get_json<T: DeserializeOwned>(client: &Client, key: &str) -> Result<Option<T>> {
    let s = get(client, key).await?;
    if s.is_none() {
        return Ok(None);
    }
    let s = s.unwrap();
    let t = serde_json::from_str(&s).map_err(Error::from)?;
    Ok(t)
}
pub async fn set_json_ex<T: Serialize>(
    client: &Client,
    key: &str,
    value: &T,
    sec: usize,
) -> Result<()> {
    let v = serde_json::to_string(value).map_err(Error::from)?;
    set_ex(client, key, &v, sec).await
}

/// 从redis获取数据
pub async fn get(client: &Client, key: &str) -> Result<Option<String>> {
    let mut conn = get_conn(client).await?;
    let s: Option<String> = conn.get(key).await.map_err(Error::from)?;
    Ok(s)
}

/// 判断指定的键是否存在于redis
pub async fn is_exists(client: &Client, key: &str) -> Result<bool> {
    let mut conn = get_conn(client).await?;
    let r = conn.exists(key).await.map_err(Error::from)?;
    Ok(r)
}

/// 删除指定的键
pub async fn del(client: &Client, key: &str) -> Result<()> {
    let mut conn = get_conn(client).await?;
    conn.del(key).await.map_err(Error::from)
}

pub async fn keys(client: &Client, key: &str) -> Result<Vec<String>> {
    let mut conn = get_conn(client).await?;
    let r = conn.keys(key).await.map_err(Error::from)?;
    Ok(r)
}
