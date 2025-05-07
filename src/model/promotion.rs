use axum_rs_derive::Db;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, Db)]
#[db(table = promotions, pk = id)]
pub struct Promotion {
    #[db(find_opt)]
    #[db(skip_update)]
    pub id: String,
    /// 名称
    #[db(list_opt)]
    #[db(list_opt_like)]
    pub name: String,
    /// 内容
    pub content: String,
    /// 链接
    pub url: String,
    /// 图片
    pub img: String,
    /// 创建时间
    #[db(skip_update)]
    pub dateline: DateTime<Local>,
}
