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

    pub pin: u8,
}

#[derive(Deserialize)]
pub struct List2Admin {
    pub title: Option<String>,
    pub slug: Option<String>,
    pub subject_name: Option<String>,
    pub try_readable: Option<bool>,
    pub is_del: Option<bool>,
    pub page: u32,
    pub page_size: u32,
}

#[derive(Deserialize, Validate)]
pub struct Update {
    #[validate(range(min = 1))]
    pub id: u64,

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
    pub pin: u8,
}

#[derive(Deserialize)]
pub struct List2Web {
    pub title: Option<String>,
    pub subject_name: Option<String>,
    pub subject_slug: Option<String>,
    pub tag_name: Option<String>,
    pub order_by_hit: Option<bool>,
    pub asc_order: Option<bool>,
    pub order_by_pin: Option<bool>,
    pub page: u32,
    pub page_size: u32,
}

#[derive(Deserialize)]
pub struct Detail {
    pub slug: String,
    pub subject_slug: String,
}
