use axum_rs_derive::Db;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize, sqlx::FromRow, Db)]
#[db(table = topic_tags, pk = id)]
pub struct TopicTag {
    #[db(skip_update)]
    #[db(find_opt)]
    pub id: String,

    #[db(find_opt)]
    #[db(list_opt)]
    pub topic_id: String,

    #[db(find_opt)]
    #[db(list_opt)]
    pub tag_id: String,
}
