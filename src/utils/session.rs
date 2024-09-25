use chrono::{DateTime, Local};

use crate::Result;

/// 生成令牌
pub fn token(id: &str, key: &str, is_admin: bool) -> Result<(String, DateTime<Local>)> {
    let now = Local::now();
    let data = format!("{id}-{is_admin}-{now}");
    let t = super::hash::sha256_with_key(&data, key)?;
    Ok((t, now))
}
