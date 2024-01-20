use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

#[derive(Debug, Deserialize, Serialize)]
struct Config {
    github: String,
    email: String,
}

fn read_config() -> Result<Config, Box<dyn std::error::Error>> {
    let config_json = include_str!("config.json");
    let config: Config = serde_json::from_str(&config_json)?;

    Ok(config)
}

pub fn get_contacts() -> &'static String {
    static CONTACTS: OnceLock<String> = OnceLock::new();
    CONTACTS.get_or_init(|| {
        match read_config() {
            Ok(config) => format!(
                r#"Contact info and links -

  <a href="mailto:{}" target="_blank" style="color:SpringGreen;font-weight:500;">email</a>: {}
  <a href="https://github.com/{}" target="_blank" style="color:SpringGreen;font-weight:500;">github</a>: github.com/{}"#,
                config.email, config.email, config.github, config.github
            ),
            Err(_) => String::from(
                r#"<span style="color:Red;font-weight:500;">Error reading config.json</span>"#,
            ),
        }
    })
}
