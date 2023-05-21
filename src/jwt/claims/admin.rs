use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct AdminClaimsData {
    pub id: u32,
    pub username: String,
}
