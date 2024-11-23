use axum_rs_derive::Db;
use serde::{Deserialize, Serialize};

use super::pagination::Paginate;

#[derive(Debug, Default, Deserialize, Serialize, sqlx::FromRow, Db)]
#[db(table = tags, pk = id)]
pub struct Tag {
    #[db(skip_update)]
    #[db(find_opt)]
    pub id: String,

    #[db(exists)]
    #[db(find_opt)]
    #[db(list_opt)]
    #[db(list_opt_like)]
    pub name: String,

    #[db(find_opt)]
    #[db(list_opt)]
    pub is_del: bool,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct TagWithTopicCount {
    #[serde(flatten)]
    pub tag: Tag,
    pub topic_count: i64,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct TagWithTopicListAndCount {
    pub tag_with_topic_count: TagWithTopicCount,
    pub topic_paginate: Paginate<super::topic_views::TopicSubjectWithTags>,
}
