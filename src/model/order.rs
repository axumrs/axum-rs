use axum_rs_derive::Db;
use chrono::{DateTime, Local};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize, sqlx::Type)]
#[sqlx(type_name = "order_status")]
pub enum Status {
    #[default]
    Pending,
    Finished,
    Cancelled,
    Closed,
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Default, Deserialize, Serialize, sqlx::FromRow, Db)]
#[db(table = orders, pk = id)]
pub struct Order {
    #[db(find)]
    #[db(skip_update)]
    pub id: String,

    #[db(find)]
    #[db(list_opt)]
    #[db(skip_update)]
    pub user_id: String,

    pub amount: Decimal,
    pub actual_amount: Decimal,

    #[db(find_opt)]
    #[db(list_opt)]
    pub status: Status,
    pub snapshot: String,
    pub allow_pointer: bool,
    pub dateline: DateTime<Local>,
}

impl Order {
    pub fn to_snapshot(&self) -> Vec<OrderSnapshot> {
        serde_json::from_str(&self.snapshot).unwrap_or_default()
    }
    pub fn snapshot_to_str(snapshot_list: &Vec<OrderSnapshot>) -> String {
        serde_json::json!(snapshot_list).to_string()
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct OrderSnapshot {
    pub service: OrderSnapshotService,
    pub user: super::user::User,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct OrderSnapshotService {
    #[serde(flatten)]
    pub service: super::service::Service,
    pub actual_price: Decimal,
    pub amount: Decimal,
    pub actual_amount: Decimal,
    pub discount: i16,
    pub num: i16,
}
