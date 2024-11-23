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

pub fn get_ip(headers: &HeaderMap) -> &str {
    let cf_connection_ip = get_header_opt(&headers, "CF-CONNECTING-IP").unwrap_or_default();
    let forwarded_for = get_header_opt(&headers, "X-FORWARDED-FOR").unwrap_or_default();
    let real_ip = get_header_opt(&headers, "X-REAL-IP").unwrap_or_default();

    if !cf_connection_ip.is_empty() {
        return cf_connection_ip;
    }

    if !forwarded_for.is_empty() {
        let forwarded_for_arr = forwarded_for.split(",").collect::<Vec<_>>();
        return forwarded_for_arr.get(0).copied().unwrap_or(real_ip);
    }

    real_ip
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
