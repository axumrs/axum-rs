use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct ChangePassword {
    #[validate(length(min = 6))]
    pub password: String,
    #[validate(length(min = 6))]
    pub new_password: String,
    #[validate(length(min = 6))]
    pub re_password: String,
}

#[derive(Deserialize, Validate)]
pub struct UpdateProfile {
    #[validate(length(min = 6))]
    pub password: String,

    #[validate(range(min = 1, max = 5))]
    pub allow_device_num: i16,

    #[validate(range(min = 20, max = 1440))]
    pub session_exp: i16,
}
