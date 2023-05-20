use serde::Serialize;

#[derive(Serialize)]
pub struct AuthBody {
    pub token: String,
    pub token_type: &'static str,
}

impl AuthBody {
    pub fn new(token: String) -> Self {
        Self {
            token,
            token_type: "Bearer",
        }
    }
}
