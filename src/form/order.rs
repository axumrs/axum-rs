use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct Create {
    // #[validate(range(min = 1))]
    // pub user_id: u32,
    #[validate(range(min = 1))]
    pub price: u32,
    #[validate(length(min = 1))]
    pub snap: String,
}
