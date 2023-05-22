use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct AdminLogin {
    #[validate(length(max = 50))]
    pub username: String,
    #[validate(length(min = 6))]
    pub password: String,
    #[validate(length(min = 1))]
    pub response: String,
}

#[derive(Deserialize, Validate)]
pub struct UserRegister {
    #[validate(
        email(message = "请输入正确的邮箱"),
        length(max = 255, message = "邮箱超过最大长度")
    )]
    pub email: String,
    #[validate(length(min = 3, max = 30, message = "昵称长度在3-30之间"))]
    pub nickname: String,
    #[validate(length(min = 6, message = "密码最少需要6个字符"))]
    pub password: String,
    #[validate(must_match(other = "password", message = "两次输入的密码不一致"))]
    pub re_password: String,
    #[validate(length(min = 1, message = "请完成人机验证"))]
    pub response: String,
}
#[derive(Deserialize, Validate)]
pub struct UserLogin {
    #[validate(
        email(message = "请输入正确的邮箱"),
        length(max = 255, message = "邮箱超过最大长度")
    )]
    pub email: String,

    #[validate(length(min = 6, message = "密码最少需要6个字符"))]
    pub password: String,

    #[validate(length(min = 1, message = "请完成人机验证"))]
    pub response: String,

    #[validate(length(min = 1, message = "无法获取IP"))]
    pub ip: String,
}
