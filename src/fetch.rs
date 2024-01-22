use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

pub const READ_JSON_ERROR: &str =
    r#"<span style="color:Red;font-weight:500;">Error reading config.json</span>"#;

#[derive(Deserialize, Serialize, Clone)]
struct Config {
    github: String,
    email: Option<String>,
    linkedin: Option<String>,
    twitter: Option<String>
}

#[allow(dead_code)]
struct User {
    name: Option<String>,
    bio: Option<String>,
    public_repos: u32,
    company: Option<String>,
    followers: u32,
    following: u32,
    created_at: String,
    stars: u32,
    forks: u32,
}

fn read_config() -> Option<Config> {
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
        match read_config() {
            Some(config) => 
            {
                let github = format!(
r#"Contact info and links -

  <a href="https://github.com/{}" target="_blank" style="color:MediumSlateBlue;font-weight:500;">Github</a>: github.com/{}"#,config.github,config.github
                );

                let email = config.email.map(|email| {
                    format!(
            r#"
  <a href="mailto:{}" target="_blank" style="color:OrangeRed;font-weight:500;">Email</a>: {}"#,
            email, email
                    )
                });

                let linkedin = config.linkedin.map(|linkedin| {
                    format!(
            r#"
  <a href="https://www.linkedin.com/{}" target="_blank" style="color:DodgerBlue;font-weight:500;">LinkedIn</a>: linkedin.com/{}"#,
            linkedin, linkedin
                    )
                });

                let twitter = config.twitter.map(|twitter| {
                    format!(
            r#"
  <a href="https://www.twitter.com/{}" target="_blank" style="color:RoyalBlue;font-weight:500;">Twitter/X</a>: @{}"#,
            twitter, twitter
                    )
                });

                format!("{}{}{}{}", github, email.unwrap_or_default(), linkedin.unwrap_or_default(), twitter.unwrap_or_default())
            },
            _ => String::from(READ_JSON_ERROR),
        }
    })
}
