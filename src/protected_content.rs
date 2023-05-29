use lazy_static::lazy_static;
use rand::Rng;
use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::{config, model, rdb, Result};

lazy_static! {
    static ref PATTERN: &'static str = r"(?sm)<(p|pre)>(.+?)</(p|pre)>";
    static ref RE: Regex = Regex::new(*PATTERN).unwrap();
    static ref PROTECTED_CONTENT_ELEMENT: &'static str =
        r#"<div class="protected_content" id="protected_content_[ID]"></div>"#;
    static ref PROTECTED_CONTENT_PLACEHOLDER: &'static str = "--内容保护--";
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProtectedContent {
    pub id: String,
    pub content: String,
}
/// 保护内容，返回 Result<(保护后的内容, Vec<保护内容的id>)>
pub async fn protected_content(
    html: &str,
    cfg: &config::Config,
    user_type: &Option<model::UserTypes>,
    rdc: &redis::Client,
) -> Result<Option<(String, Vec<String>)>> {
    // 是否需要内容保护
    let need_protect = if let Some(user_type) = user_type {
        match user_type {
            &model::UserTypes::Normal => true,
            &model::UserTypes::Subscriber => false,
        }
    } else {
        true
    };

    if !need_protect {
        return Ok(None);
    }

    // 使用正则捕获
    let mut pcl = vec![];
    for cap in (*RE).captures_iter(html) {
        let id = xid::new().to_string();
        let content = cap[0].to_string();
        pcl.push(ProtectedContent { id, content });
    }

    let pcl_len = pcl.len();

    // 如果捕获的内容小于配置的内容, 直接返回None，不做处理
    if pcl_len < cfg.protected_topic.min_content_paragraph_num as usize {
        return Ok(None);
    }

    // 保护的数量
    let protect_num = match pcl_len {
        0..=2 => 0,
        3..=5 => 1,
        6..=8 => 2,
        _ => cfg.protected_topic.max_paragraph_num,
    };

    // 随机选择要保护的幸运儿
    let mut protect_idxs = Vec::with_capacity(protect_num as usize);
    for _ in 0..protect_num {
        loop {
            let tmp: usize = rand::thread_rng().gen_range(0..100);
            let tmp = tmp % pcl_len;
            if !in_idx(&protect_idxs[..], tmp) {
                protect_idxs.push(tmp);
                break;
            }
        }
    }
    protect_idxs.sort();

    tracing::debug!(
        "pcl_len {}, protect_num {}, protect_idxs {:?}",
        pcl_len,
        protect_num,
        protect_idxs
    );

    // 替换
    let html = (*RE).replace_all(html, *PROTECTED_CONTENT_PLACEHOLDER);

    let mut procted_line_idx = 0usize;
    let mut out_html = Vec::with_capacity(html.lines().count());
    let mut out_ids: Vec<String> = Vec::with_capacity(protect_idxs.len());

    for line in html.lines() {
        if line == *PROTECTED_CONTENT_PLACEHOLDER {
            if let Some(pc) = pcl.pop() {
                if in_idx(&protect_idxs[..], procted_line_idx) {
                    out_html.push(
                        (*PROTECTED_CONTENT_ELEMENT)
                            .to_string()
                            .replace("[ID]", &pc.id),
                    );
                    if !in_idx(&out_ids[..], pc.id.clone()) {
                        out_ids.push(pc.id.clone());
                    }
                    let rds_key = rdb::protected_topic_keyname(cfg, &pc.id);
                    rdb::set_ex(
                        rdc,
                        &rds_key,
                        &pc.content,
                        cfg.protected_topic.redis_expired as usize * 60,
                    )
                    .await?;
                } else {
                    out_html.push(pc.content.clone());
                }
            }
            procted_line_idx += 1;
        } else {
            out_html.push(line.to_string());
        }
    }

    let out_html: String = out_html.join("\n");

    // tracing::debug!("{}", out_html);
    tracing::debug!("{:?}", out_ids);
    Ok(Some((out_html, out_ids)))
}

fn in_idx<T: Eq + PartialOrd>(idx: &[T], i: T) -> bool {
    for ii in idx.iter() {
        if *ii == i {
            return true;
        }
    }
    false
}
