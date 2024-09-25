use anyhow::anyhow;
use lazy_static::lazy_static;
use scraper::{Html, Selector};

use crate::{model, Error, Result};

lazy_static! {
    static ref DOM_ROOT_ID: &'static str = "__AXUM__";
    static ref ELEMENT_SELECTOR: String = "p,h1,h2,h3,h4,h5,h6,pre,table,blockquote,ul,ol"
        .split(",")
        .collect::<Vec<_>>()
        .into_iter()
        .map(|s| format!("#{} > {}", (*DOM_ROOT_ID), s))
        .collect::<Vec<_>>()
        .join(",");
}

pub fn sections(html: &str) -> Result<Vec<(i32, String)>> {
    let html = format!(r#"<div id="{}">{}</div>"#, (*DOM_ROOT_ID), html);
    let fragment = Html::parse_fragment(&html);
    let mut v = vec![];

    let selector =
        Selector::parse(ELEMENT_SELECTOR.as_str()).map_err(|e| Error::from(anyhow!("{:?}", e)))?;
    for (idx, con) in fragment.select(&selector).enumerate() {
        v.push((idx as i32, con.html()));
    }
    Ok(v)
}

#[cfg(test)]
mod test {
    fn read_data(filename: &str) -> std::io::Result<String> {
        let path = format!("test-sections-data/{}", filename);
        std::fs::read_to_string(&path)
    }
    #[test]
    fn test_cap_topic_sects() {
        let html = read_data("a.txt").unwrap();
        let ss = super::sections(&html).unwrap();
        // for (sort, con) in ss {
        //     println!("{}", &con);
        //     println!("==={}==", sort);
        // }
        let data = ss.into_iter().map(|s| s.1).collect::<Vec<_>>();
        let data = data.join("\n");
        std::fs::write("/tmp/a.txt", &data).unwrap();
    }
}
