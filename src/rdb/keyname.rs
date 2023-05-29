use crate::Config;

pub fn keyname(names: &[&str]) -> String {
    names.join("").to_string()
}

pub fn user_keyname(cfg: &Config, prefix: &str, key: &str) -> String {
    keyname(&[&cfg.redis.prefix, &cfg.users.redis_prefix, prefix, key])
}
pub fn protected_topic_keyname(cfg: &Config, key: &str) -> String {
    keyname(&[&cfg.redis.prefix, &cfg.protected_topic.redis_prefix, key])
}
