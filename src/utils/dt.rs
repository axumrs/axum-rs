use chrono::{DateTime, Local, NaiveDateTime, TimeZone};

use crate::{Error, Result};

pub fn naive_to_local(n: &NaiveDateTime) -> Result<DateTime<Local>> {
    match Local.from_local_datetime(n) {
        chrono::offset::LocalResult::Single(v) => Ok(v),
        chrono::offset::LocalResult::Ambiguous(v, _) => Ok(v),
        chrono::offset::LocalResult::None => Err(Error::new("无法解析日期时间")),
    }
}
pub fn parse(dt_str: &str) -> Result<DateTime<Local>> {
    let nd = NaiveDateTime::parse_from_str(dt_str, "%Y-%m-%d %H:%M:%S").map_err(Error::from)?;
    naive_to_local(&nd)
}
