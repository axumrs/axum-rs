use serde::Deserialize;
use validator::Validate;
#[derive(Deserialize, Validate)]
pub struct Create {
    #[validate(length(max = 255))]
    pub title: String,

    #[validate(range(min = 1))]
    pub subject_id: u32,

    #[validate(length(max = 100))]
    pub slug: String,

    #[validate(length(max = 255))]
    pub summary: String,

    #[validate(length(max = 50))]
    pub author: String,

    #[validate(length(max = 50))]
    pub src: String,

    pub try_readable: bool,

    #[validate(length(max = 100))]
    pub cover: String,

    #[validate(length(min = 1))]
    pub md: String,

    pub tags: Vec<String>,
}
