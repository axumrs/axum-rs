use anyhow::anyhow;
use lazy_static::lazy_static;
use scraper::{Html, Selector};

use crate::{model, Error, Result};

lazy_static! {
    static ref DOM_ROOT_ID: &'static str = "__AXUM__";
    static ref ELEMENT_SELECTOR: String = "p,h1,h2,h3,h4,h5,h6,pre,table,blockquote,ul,ol,dl,div"
        .split(",")
        .collect::<Vec<_>>()
        .into_iter()
        .map(|s| format!("#{} > {}", (*DOM_ROOT_ID), s))
        .collect::<Vec<_>>()
        .join(",");
}

pub fn sections(
    t: &model::topic::Topic,
    hash_secret_key: &str,
) -> Result<Vec<model::topic::TopicSection>> {
    let html = super::md::to_html(&t.md);
    let secs = html_sections(&html)?;

    let tsc = secs
        .into_iter()
        .map(|(sort, id, content)| {
            let hash = super::hash::sha256_with_key(&content, hash_secret_key).unwrap_or_default();
            model::topic::TopicSection {
                id,
                topic_id: t.id.clone(),
                sort,
                hash,
                content,
            }
        })
        .collect::<Vec<_>>();

    Ok(tsc)
}

pub fn html_sections(html: &str) -> Result<Vec<(i32, String, String)>> {
    let html = format!(r#"<div id="{}">{}</div>"#, (*DOM_ROOT_ID), html);
    let fragment = Html::parse_fragment(&html);
    let mut v = vec![];

    let selector =
        Selector::parse(ELEMENT_SELECTOR.as_str()).map_err(|e| Error::from(anyhow!("{:?}", e)))?;
    for (idx, con) in fragment.select(&selector).enumerate() {
        let id = super::id::new();
        let tag_name = con.value().name();
        let inner_html = con.inner_html();
        let el = format!(r#"<{tag_name} data-section="{id}">{inner_html}</{tag_name}>"#);
        v.push((idx as i32, id, el));
    }
    Ok(v)
}

#[cfg(test)]
mod test {
    use crate::{model, utils};

    fn read_data(filename: &str) -> std::io::Result<String> {
        let path = format!("test-sections-data/{}", filename);
        std::fs::read_to_string(&path)
    }
    #[test]
    fn test_utils_html_sections() {
        let html = read_data("b.txt").unwrap();
        let ss = super::html_sections(&html).unwrap();
        // for (sort, con) in ss {
        //     println!("{}", &con);
        //     println!("==={}==", sort);
        // }
        let data = ss.into_iter().map(|s| s.2).collect::<Vec<_>>();
        let data = data.join("\n");
        std::fs::write("/tmp/b.txt", &data).unwrap();
    }
    #[test]
    fn test_utils_topic_sections() {
        let md = read_data("b.md").unwrap();
        let topic = model::topic::Topic {
            id: utils::id::new(),
            md,
            ..Default::default()
        };
        let tcs = super::sections(&topic, "").unwrap();
        let content = tcs.iter().map(|tc| tc.content.clone()).collect::<Vec<_>>();
        let content = content.join("\n");
        std::fs::write("/tmp/b.txt", &content).unwrap();
    }
}
