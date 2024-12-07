use axum::{extract::State, http::HeaderMap, response::IntoResponse};
use chrono::Local;

use crate::{mid::IpAndUserAgent, model, resp, service, ArcAppState, Error, Result};
use rss::{CategoryBuilder, ChannelBuilder, ImageBuilder, ItemBuilder};

use super::{get_pool, log_error};

pub async fn ping(
    State(state): State<ArcAppState>,
    ip_and_user_agent: IpAndUserAgent,
) -> resp::JsonResp<String> {
    resp::ok(format!(
        "[PONG] prefix: {}, client: {:?}",
        &state.cfg.web.prefix, &ip_and_user_agent
    ))
}

pub async fn rss(State(state): State<ArcAppState>) -> Result<axum::response::Response> {
    let handler_name = "web/rss";
    let p = get_pool(&state);
    let host = &state.cfg.host;

    let data = service::topic::list_all_opt(
        &*p,
        &model::topic_views::VTopicSubjectListAllFilter {
            limit: Some(30),
            order: Some("id DESC".into()),
            title: None,
            subject_id: None,
            slug: None,
            is_del: Some(false),
            subject_slug: None,
            subject_is_del: Some(false),
            status: None,
            v_topic_subject_list_all_between_datelines: None,
        },
    )
    .await
    .map_err(Error::from)
    .map_err(log_error(handler_name))?;

    let items = data
        .into_iter()
        .map(|i| {
            ItemBuilder::default()
                .title(Some(i.topic_subjects.title))
                .pub_date(Some(i.topic_subjects.dateline.to_rfc2822()))
                .link(Some(format!(
                    "{}/topic/{}/{}",
                    host, &i.topic_subjects.subject_slug, &i.topic_subjects.slug
                )))
                .category(
                    CategoryBuilder::default()
                        .name(&i.topic_subjects.name)
                        // .domain(format!("{}/subject/{}", host, &i.topic_subjects.slug))
                        .build(),
                )
                .description(Some(i.topic_subjects.summary))
                .build()
        })
        .collect::<Vec<_>>();

    let mut channel = ChannelBuilder::default()
        .title("AXUM中文网")
        .link(host)
        .description("AXUM中文网为你提供了企业级axum Web开发中所需要的大部分知识。")
        .image(
            ImageBuilder::default()
                .url("https://file.axum.eu.org/asset/logo.png")
                .title("AXUM中文网")
                .link(host)
                .build(),
        )
        .copyright(format!("2021-present AXUM中文网 {}", host))
        .pub_date(Local::now().to_rfc2822())
        .build();
    channel.items.extend(items);

    channel.write_to(::std::io::sink()).unwrap(); // // write to the channel to a writer
    let string = channel.to_string(); // convert the channel to a string

    let mut header = HeaderMap::new();
    header.insert("Content-Type", "text/xml".parse().unwrap());
    Ok((header, string).into_response())
}
