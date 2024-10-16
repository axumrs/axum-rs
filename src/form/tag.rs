use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize)]
pub struct ListForAdmin {
    #[serde(flatten)]
    pub pq: super::PageQueryStr,
    pub name: Option<String>,
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

#[derive(Deserialize, Validate)]
pub struct Add {
    #[validate(length(min = 1, max = 100))]
    pub name: String,
}
#[derive(Deserialize, Validate)]
pub struct Edit {
    #[validate(length(min = 20, max = 20))]
    pub id: String,

    #[serde(flatten)]
    pub base: Add,
}
