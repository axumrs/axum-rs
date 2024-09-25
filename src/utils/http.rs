use axum::http::{
    header::{AsHeaderName, AUTHORIZATION, USER_AGENT},
    HeaderMap,
};

pub fn get_header_opt(headers: &HeaderMap, key: impl AsHeaderName) -> Option<&str> {
    headers.get(key).and_then(|v| v.to_str().ok())
}

pub fn get_user_agent_opt(headers: &HeaderMap) -> Option<&str> {
    get_header_opt(headers, USER_AGENT)
}

pub fn get_user_agent(headers: &HeaderMap) -> &str {
    get_user_agent_opt(headers).unwrap_or_default()
}

pub fn get_ip_opt(headers: &HeaderMap) -> Option<&str> {
    let cf_connection_ip = get_header_opt(&headers, "CF-CONNECTING-IP");
    let forwarded_for = get_header_opt(&headers, "X-FORWARDED-FOR");
    let real_ip = get_header_opt(&headers, "X-REAL-IP");

    if let Some(v) = cf_connection_ip {
        return Some(v);
    }

    if let Some(v) = forwarded_for {
        if v.contains(",") {
            let arr = v.split(",").collect::<Vec<_>>();
            return match arr.get(0).copied() {
                Some(v) => Some(v),
                None => real_ip,
            };
        }
    }

    real_ip
}

pub fn get_ip(headers: &HeaderMap) -> &str {
    get_ip_opt(headers).unwrap_or("0.0.0.0")
}

pub fn get_cf_location(headers: &HeaderMap) -> &str {
    get_header_opt(headers, "CF-IPCOUNTRY").unwrap_or_default()
}

pub fn get_auth(headers: &HeaderMap) -> Option<&str> {
    get_header_opt(headers, AUTHORIZATION)
}

pub fn get_auth_token(headers: &HeaderMap) -> Option<&str> {
    let v = get_auth(headers);

    if let Some(v) = v {
        if let Some(v) = v.strip_prefix("Bearer ") {
            return Some(v);
        }
    }

    None
}
