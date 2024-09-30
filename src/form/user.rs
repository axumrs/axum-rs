use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct AddForm {
    #[validate(email)]
    #[validate(length(max = 255))]
    pub email: String,

    #[validate(length(min = 3, max = 30))]
    pub nickname: String,

    #[validate(length(min = 6))]
    pub password: String,

    #[validate(length(min = 6))]
    pub re_password: String,
}
