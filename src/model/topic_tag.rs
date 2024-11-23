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

#[derive(Debug, Default, Deserialize, Serialize, sqlx::FromRow, Db)]
#[db(table = v_topic_tag_with_tags, pk = id, is_view)]
pub struct VTopicTagWithTag {
    // tag
    /// 标签ID
    #[db(find_opt)]
    pub id: String,

    /// 标签名称
    #[db(find_opt)]
    #[db(list_opt)]
    pub name: String,

    /// 标签是否删除
    #[db(find_opt)]
    #[db(list_opt)]
    pub is_del: bool,

    // topic_tags
    /// 关联表ID
    pub topic_tag_id: String,

    /// 文章ID
    #[db(list)]
    pub topic_id: String,
}
