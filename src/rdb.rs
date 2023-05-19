//! redis 操作

use redis::{aio::Connection, AsyncCommands, Client};

use crate::{Error, Result};

/// 获取连接
async fn get_conn(client: &Client) -> Result<Connection> {
    client.get_async_connection().await.map_err(Error::from)
}

/// 将数据写入 redis
pub async fn set(client: &Client, key: &str, value: &str, sec: usize) -> Result<()> {
    let mut conn = get_conn(client).await?;
    conn.set_ex(key, value, sec).await.map_err(Error::from)
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
