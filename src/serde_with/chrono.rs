use chrono::{DateTime, Local, TimeZone};
use serde::{self, Deserialize, Deserializer};

const FORMAT: &str = "%Y-%m-%d %H:%M:%S";

pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Local>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Local
        .datetime_from_str(&s, FORMAT)
        .map_err(serde::de::Error::custom)
}
