use uaparser::{Parser, UserAgentParser};

use crate::{Error, Result};

const DEFAULT_YAML_PATH: &str = "./ua-reg.yaml";

#[derive(Default)]
pub struct UserAgentInfo {
    pub device: String,
    pub os: String,
    pub browser: String,
}

pub fn get_parser_from(yaml_path: &str) -> Result<UserAgentParser> {
    UserAgentParser::from_yaml(yaml_path).map_err(Error::from)
}

pub fn get_parser() -> Result<UserAgentParser> {
    get_parser_from(DEFAULT_YAML_PATH)
}

pub fn parse_from(yaml_path: &str, user_agent: &str) -> Result<UserAgentInfo> {
    let client = get_parser_from(yaml_path)?.parse(user_agent);
    // tracing::debug!("{:?}", client);

    Ok(UserAgentInfo {
        device: client.device.family.to_string(),
        os: format!(
            "{} {}",
            client.os.family,
            client.os.major.unwrap_or_default()
        ),
        browser: format!(
            "{} {}",
            client.user_agent.family,
            client.user_agent.major.unwrap_or_default()
        ),
    })
}

pub fn parse(user_agent: &str) -> Result<UserAgentInfo> {
    parse_from(DEFAULT_YAML_PATH, user_agent)
}
