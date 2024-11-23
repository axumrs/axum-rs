use serde::Serialize;

use crate::model;

#[derive(Serialize)]
pub struct Detail {
    pub subject: model::subject::Subject,
    pub topic_list: Vec<model::topic_views::TopicSubjectWithTags>,
}
