use serde::{de::DeserializeOwned, Serialize};

use crate::{config, Result};

use super::{AuthBody, Claims, Key};

pub fn encode<T>(cfg: &config::Jwt, data: T) -> Result<AuthBody>
where
    T: Serialize + DeserializeOwned,
{
    let key = get_key(cfg);
    let claims = Claims::from_cfg(cfg, data);
    claims.token(&key)
}

pub fn decode<T>(token: &str, cfg: &config::Jwt) -> Result<Claims<T>>
where
    T: Serialize + DeserializeOwned,
{
    let key = get_key(cfg);
    Claims::<T>::from_token(token, &key)
}

fn get_key(cfg: &config::Jwt) -> Key {
    Key::from_cfg(cfg)
}
