use redis::Client;

use crate::{jwt, Config, Result};

pub async fn get_allow_drive(client: &Client, cfg: &Config, email: &str) -> Result<u8> {
    let key = super::user_keyname(cfg, &cfg.users.redis_allow_drive_prefix, email);
    let r = super::get(client, &key).await?;
    let n = match r {
        None => 1,
        Some(s) => s.parse().unwrap_or(1),
    };
    Ok(n)
}

pub async fn set_allow_drive(client: &Client, cfg: &Config, email: &str, n: u8) -> Result<()> {
    let key = super::user_keyname(cfg, &cfg.users.redis_allow_drive_prefix, email);
    let ns = n.to_string();
    super::set(client, &key, &ns).await
}

pub async fn get_jwt_exp(client: &Client, cfg: &Config, email: &str) -> Result<u8> {
    let key = super::user_keyname(cfg, &cfg.users.redis_jwt_exp_prefix, email);
    let r = super::get(client, &key).await?;
    let n = match r {
        None => 0,
        Some(s) => s.parse().unwrap_or(0),
    };
    Ok(n)
}

pub async fn set_jwt_exp(client: &Client, cfg: &Config, email: &str, n: u8) -> Result<()> {
    let key = super::user_keyname(cfg, &cfg.users.redis_jwt_exp_prefix, email);
    let ns = n.to_string();
    super::set(client, &key, &ns).await
}

pub async fn count_online(client: &Client, cfg: &Config, email: &str) -> Result<u8> {
    Ok(get_online_keys(client, cfg, email).await?.len() as u8)
}

pub async fn get_online_keys(client: &Client, cfg: &Config, email: &str) -> Result<Vec<String>> {
    let key = format!("{}::*", email);
    let key = super::user_keyname(cfg, &cfg.users.redis_online_prefix, &key);
    super::keys(client, &key).await
}

pub async fn get_online_list(
    client: &Client,
    cfg: &Config,
    email: &str,
) -> Result<Vec<jwt::UserClaimsData>> {
    let keys = get_online_keys(client, cfg, email).await?;
    let mut cds = Vec::with_capacity(keys.len());
    for key in keys {
        let cd = super::get_json(client, &key).await?;
        if let Some(cd) = cd {
            cds.push(cd);
        }
    }
    Ok(cds)
}

pub async fn set_online(
    client: &Client,
    cfg: &Config,
    email: &str,
    cd: &jwt::UserClaimsData,
    exp_mins: u32,
    online_id: &str,
) -> Result<()> {
    let key = format!("{}::{}", email, online_id);
    let key = super::user_keyname(cfg, &cfg.users.redis_online_prefix, &key);
    let secs = (exp_mins * 60) as usize;
    super::set_json_ex(client, &key, cd, secs).await
}

pub async fn del_online(client: &Client, cfg: &Config, email: &str, online_id: &str) -> Result<()> {
    let key = format!("{}::{}", email, online_id);
    let key = super::user_keyname(cfg, &cfg.users.redis_online_prefix, &key);
    super::del(client, &key).await
}
