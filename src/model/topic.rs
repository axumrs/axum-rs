use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize, sqlx::FromRow)]
pub struct Topic {
    pub id: u64,
    pub title: String,
    pub subject_id: u32,
    pub slug: String,
    pub summary: String,
    pub author: String,
    pub src: String,
    pub hit: u64,
    pub dateline: chrono::DateTime<chrono::Local>,
    pub try_readable: bool,
    pub is_del: bool,
    pub cover: String,
    pub pin: u8,
}

#[derive(Debug, Default, Deserialize, Serialize, sqlx::FromRow)]
pub struct TopicContent {
    pub topic_id: u64,
    pub md: String,
    pub html: String,
}

#[derive(Debug, Default, Deserialize, Serialize, sqlx::FromRow)]
pub struct Topic2AdminList {
    pub id: u64,
    pub title: String,
    pub slug: String,
    pub hit: u64,
    pub dateline: chrono::DateTime<chrono::Local>,
    pub try_readable: bool,
    pub is_del: bool,
    pub cover: String,
    pub subject_name: String,
    pub subject_slug: String,
    pub pin: u8,
}

#[derive(Default)]
pub struct Topic2AdminListWith {
    pub title: Option<String>,
    pub slug: Option<String>,
    pub subject_name: Option<String>,
    pub try_readable: Option<bool>,
    pub is_del: Option<bool>,
    pub page: u32,
    pub page_size: u32,
}

#[derive(Debug, Default, Deserialize, Serialize, sqlx::FromRow)]
pub struct Topic2Edit {
    pub id: u64,
    pub title: String,
    pub subject_id: u32,
    pub slug: String,
    pub summary: String,
    pub author: String,
    pub src: String,
    pub cover: String,
    pub md: String,
    pub try_readable: bool,
    pub pin: u8,
}

#[derive(Debug, Default, Deserialize, Serialize, sqlx::FromRow)]
pub struct Topic2WebList {
    pub id: u64,
    pub title: String,
    pub slug: String,
    pub summary: String,
    pub cover: String,
    pub try_readable: bool,
    pub subject_name: String,
    pub subject_slug: String,
    pub tag_names: String,
    pub pin: u8,
    pub subject_pin: u8,
}

#[derive(Default)]
pub struct Topic2WebListWith {
    pub title: Option<String>,
    pub subject_name: Option<String>,
    pub subject_slug: Option<String>,
    pub tag_name: Option<String>,
    pub order_by_hit: bool,
    pub asc_order: bool,
    pub order_by_pin: bool,
    pub page: u32,
    pub page_size: u32,
}

#[derive(Debug, Default, Deserialize, Serialize, sqlx::FromRow)]
pub struct Topic2WebDetail {
    pub id: u64,
    pub title: String,
    pub slug: String,
    pub cover: String,
    pub try_readable: bool,
    pub hit: u64,
    pub dateline: chrono::DateTime<chrono::Local>,
    pub html: String,
    pub subject_name: String,
    pub subject_slug: String,
    pub tag_names: String,
    pub price: u32,
    pub subject_id: u32,
}

#[derive(Default, Serialize, Deserialize, Clone)]
pub enum ProtectedTopic2WebDetailCaptchaType {
    #[default]
    HCaptcha,
    ReCaptcha,
    Turnstile,
    WithOut,
}
#[derive(Default, Serialize)]
pub struct ProtectedTopic2WebDetail {
    pub detail: Topic2WebDetail,
    pub protect_ids: Vec<String>,
    pub captcha_type: ProtectedTopic2WebDetailCaptchaType,
}

impl ProtectedTopic2WebDetail {
    pub fn with_out(detail: Topic2WebDetail) -> Self {
        Self {
            detail,
            protect_ids: vec![],
            captcha_type: ProtectedTopic2WebDetailCaptchaType::WithOut,
        }
    }
}
