use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Config {
    github: String,
    email: String,
}

#[allow(dead_code)]
struct Github {
    name: String,
    public_repos: u32,
    followers: u32,
    following: u32,
    created_at: String,
}

fn get_config() -> Option<Config> {
    let config_json = include_str!("config.json");
    static CONFIG: OnceLock<Option<Config>> = OnceLock::new();

    CONFIG.get_or_init(|| match serde_json::from_str::<Config>(&config_json) {
        Ok(config) => Some(config),
        Err(_) => None,
    });

    CONFIG.get().cloned()?
}

pub fn get_contacts() -> &'static String {
    static CONTACTS: OnceLock<String> = OnceLock::new();
    CONTACTS.get_or_init(|| {
        match get_config() {
            Some(config) => format!(
                r#"Contact info and links -

  <a href="mailto:{}" target="_blank" style="color:SpringGreen;font-weight:500;">email</a>: {}
  <a href="https://github.com/{}" target="_blank" style="color:SpringGreen;font-weight:500;">github</a>: github.com/{}"#,
                config.email, config.email, config.github, config.github
            ),
            _ => String::from(
                r#"<span style="color:Red;font-weight:500;">Error reading config.json</span>"#,
            ),
        }
    })
}
