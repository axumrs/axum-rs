use axum_rs_derive::Db;
use chrono::{DateTime, Local};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::config;

#[derive(Debug, Default, Deserialize, Serialize, sqlx::FromRow, Db)]
#[db(table = v_topic_subjects, pk = id)]
pub struct VTopicSubject {
    #[db(find_opt)]
    pub id: String,

    #[db(list_opt)]
    #[db(list_opt_like)]
    pub title: String,

    #[db(find_opt)]
    #[db(list_opt)]
    pub subject_id: String,

    #[db(find_opt)]
    #[db(list_opt)]
    pub slug: String,

    pub summary: String,
    pub author: String,
    pub src: String,
    pub hit: i64,

    #[db(list_opt)]
    #[db(list_opt_between)]
    pub dateline: DateTime<Local>,
    pub try_readable: bool,

    #[db(find_opt)]
    #[db(list_opt)]
    pub is_del: bool,
    pub cover: String,
    pub md: String,
    pub pin: i32,

    // subject
    pub name: String,
    #[db(find_opt)]
    #[db(list_opt)]
    pub subject_slug: String,
    pub subject_summary: String,

    #[db(find_opt)]
    #[db(list_opt)]
    pub subject_is_del: bool,

    pub subject_cover: String,
    #[db(list_opt)]
    pub status: super::subject::Status,
    pub price: Decimal,
    pub subject_pin: i32,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct TopicSubjectWithTags {
    #[serde(flatten)]
    pub topic_subjects: VTopicSubject,
    pub tags: Vec<super::topic_tag::VTopicTagWithTag>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct TopicSubjectWithTagsAndSections {
    #[serde(flatten)]
    pub topic_subject_with_tags: TopicSubjectWithTags,
    pub sections: Vec<super::topic::TopicSection>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct TopicProctedMeta {
    pub ids: Vec<String>,
    pub catpcha: config::CaptchaKind,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct TopicSubjectWithTagsAndProctedSections {
    #[serde(flatten)]
    pub topic_subject_with_tags_and_sections: TopicSubjectWithTagsAndSections,
    pub protected: TopicProctedMeta,
}
