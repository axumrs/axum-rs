use axum::extract::State;
use serde::{Deserialize, Serialize};

use crate::{
    api::{get_pool, log_error},
    resp, utils, ArcAppState, Error, Result,
};

#[derive(Serialize, Default, Deserialize, sqlx::FromRow)]
pub struct Statistics {
    pub title: String,
    pub all_users: i64,
    pub today_users: i64,
    pub all_orders: i64,
    pub today_orders: i64,
    pub subject: i64,
    pub topic: i64,
    pub service: i64,
    pub purchased_service: i64,
}
pub async fn index(State(state): State<ArcAppState>) -> Result<resp::JsonResp<Statistics>> {
    let handler_name = "admin/statistics/index";
    let p = get_pool(&state);

    let (start, end) = utils::dt::today();

    let r: Vec<Statistics> = sqlx::query_as(
        r#"SELECT t.title
	,COALESCE(MAX(t.c) FILTER (WHERE title='all_users'), 0) AS all_users
	,COALESCE(max(t.c) FILTER( WHERE title='today_users'), 0) AS today_users 
	,COALESCE(max(t.c) FILTER( WHERE title='all_orders'), 0) AS all_orders 
	,COALESCE(max(t.c) FILTER( WHERE title='today_orders'), 0) AS today_orders 
	,COALESCE(max(t.c) FILTER( WHERE title='subject'), 0) AS subject 
	,COALESCE(max(t.c) FILTER( WHERE title='topic'), 0) AS topic 
	,COALESCE(max(t.c) FILTER( WHERE title='service'), 0) AS service 
	,COALESCE(max(t.c) FILTER( WHERE title='purchased_service'), 0) AS purchased_service 
FROM (
	SELECT 'all_users' AS title, COUNT(*) AS c FROM users
	UNION  ALL 
	SELECT 'today_users' AS title, COUNT(*) AS c FROM users WHERE dateline BETWEEN $1 AND $2
	UNION ALL
	SELECT 'all_orders' AS title, COUNT(*) AS c FROM orders
	UNION  ALL 
	SELECT 'today_orders' AS title, COUNT(*) AS c FROM orders WHERE dateline BETWEEN $1 AND $2
	UNION ALL
	SELECT 'subject' AS title, COUNT(*) AS c FROM subjects
	UNION ALL
	SELECT 'topic' AS title, COUNT(*) AS c FROM topics
	UNION ALL
	SELECT 'service' AS title, COUNT(*) AS c FROM services
	UNION ALL
	SELECT 'purchased_service' AS title, 0 AS c
) AS t
GROUP BY title"#,
    )
    .bind(&start)
    .bind(&end)
    .fetch_all(&*p)
    .await
    .map_err(Error::from)
    .map_err(log_error(handler_name))?;

    let mut data = Statistics::default();

    for v in r {
        match v.title.as_str() {
            "all_users" => data.all_users = v.all_users.max(0),
            "today_users" => data.today_users = v.today_users.max(0),
            "all_orders" => data.all_orders = v.all_orders.max(0),
            "today_orders" => data.today_orders = v.today_orders.max(0),
            "subject" => data.subject = v.subject.max(0),
            "topic" => data.topic = v.topic.max(0),
            "service" => data.service = v.service.max(0),
            //"purchased_service" => data.purchased_service = v.purchased_service.max(0),
            _ => {}
        }
    }

    // TODO：处理已购服务

    Ok(resp::ok(data))
}
