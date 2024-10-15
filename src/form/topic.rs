use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct GetProtectedContent {
    #[validate(length(min = 1))]
    pub ids: Vec<String>,

    #[validate(length(min = 20, max = 20))]
    pub topic_id: String,

    #[validate(length(min = 6))]
    pub captcha: String,
}

#[derive(Deserialize, Validate)]
pub struct Add {
    #[validate(length(min = 1))]
    pub tag_names: Vec<String>,

    #[validate(length(min = 1, max = 255))]
    pub title: String,

    #[validate(length(min = 20, max = 20))]
    pub subject_id: String,

    #[validate(length(min = 1, max = 100))]
    pub slug: String,

    #[validate(length(min = 1, max = 255))]
    pub summary: String,

    #[validate(length(min = 1, max = 50))]
    pub author: String,

    #[validate(length(min = 1, max = 50))]
    pub src: String,

    pub try_readable: bool,

    #[validate(length(max = 100))]
    pub cover: String,

    #[validate(length(min = 1))]
    pub md: String,

    pub pin: i32,
}

#[derive(Deserialize, Validate)]
pub struct Edit {
    #[validate(length(min = 20, max = 20))]
    pub id: String,

    #[serde(flatten)]
    pub base: Add,
}

#[derive(Deserialize)]
pub struct ListForAdmin {
    #[serde(flatten)]
    pub pq: super::PageQueryStr,

    pub title: Option<String>,
    pub subject_name: Option<String>,
    pub subject_slug: Option<String>,
    pub slug: Option<String>,
    pub is_del: Option<String>,
}

impl ListForAdmin {
    pub fn is_del(&self) -> Option<bool> {
        if let Some(ref v) = self.is_del {
            Some(v == "1")
        } else {
            None
        }
    }
}
