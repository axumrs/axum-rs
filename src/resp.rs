use axum::Json;
use serde::Serialize;

use crate::Error;

#[derive(Serialize)]
pub struct Resp<T> {
    pub code: i32,
    pub msg: String,
    pub data: T,
}

impl<T: Serialize> Resp<T> {
    pub fn new(code: i32, msg: impl ToString, data: T) -> Self {
        Self {
            code,
            msg: msg.to_string(),
            data,
        }
    }

    pub fn ok(data: T) -> Self {
        Self::new(0, "OK", data)
    }

    pub fn to_json(self) -> Json<Self> {
        Json(self)
    }
}

impl Resp<()> {
    pub fn empty_ok() -> Self {
        Self::ok(())
    }
    pub fn err(e: Error) -> Self {
        Self::new(-1, e, ())
    }
}

#[derive(Serialize)]
pub struct IDResp {
    pub id: String,
}

#[derive(Serialize)]
pub struct AffResp {
    pub aff: u64,
}

pub type JsonResp<T> = Json<Resp<T>>;
pub type JsonIDResp = JsonResp<IDResp>;
pub type JsonAffResp = JsonResp<AffResp>;

pub fn ok<T: Serialize>(data: T) -> JsonResp<T> {
    Resp::ok(data).to_json()
}

pub fn err(e: Error) -> JsonResp<()> {
    Resp::err(e).to_json()
}
