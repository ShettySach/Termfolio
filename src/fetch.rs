use serde::{Deserialize, Serialize};
use std::sync::OnceLock;
use tokio::sync::OnceCell;
use tokio::try_join;

// Error lines

pub const READ_JSON_ERROR: &str =
    r#"<span style="color:Red;font-weight:500;">Error reading config.json</span>"#;

pub const FETCH_GITHUB_ERROR: &str =
    r#"<span style="color:Red;font-weight:500;">Error fetching data from Github.</span>"#;

// Once statics

static CONFIG: OnceLock<Option<Config>> = OnceLock::new();
static ABOUT: OnceCell<String> = OnceCell::const_new();
static CONTACTS: OnceLock<String> = OnceLock::new();

// Structs

#[derive(Deserialize, Serialize, Clone)]
struct Config {
    github: String,
    email: Option<String>,
    linkedin: Option<String>,
    twitter: Option<String>,
}

#[derive(Deserialize, Serialize, Clone)]
struct UserInfo {
    name: Option<String>,
    bio: Option<String>,
    public_repos: u32,
    company: Option<String>,
    location: Option<String>,
    followers: u32,
    following: u32,
    created_at: String,
}

#[derive(Deserialize, Serialize, Clone)]
struct UserStats {
    stars: u32,
    forks: u32,
}

// Functions

pub async fn get_user() -> String {
    ABOUT.get_or_init(|| fetch_user()).await;

    match ABOUT.get() {
        Some(user) => {
            let img = include_str!("img.txt");
            let user = user.to_string();
            format!(
                r#"<div class="row">
<div class="cols">{}</div>
<div class="cols">{}</div>
</div>"#,
                img, user
            )
        }
        _ => String::from(FETCH_GITHUB_ERROR),
    }
}

pub async fn fetch_user() -> String {
    match read_config() {
        Some(config) => {
            let info_url = format!("https://api.github.com/users/{}", config.github);
            let stats_url = format!(
                "https://api.github-star-counter.workers.dev/user/{}",
                config.github
            );

            match try_join!(async { reqwest::get(&info_url).await }, async {
                reqwest::get(&stats_url).await
            }) {
                Ok((info_response, stats_response)) => {
                    if info_response.status().is_success() && stats_response.status().is_success() {
                        let user_info: UserInfo = info_response.json().await.unwrap();
                        let user_stars: UserStats = stats_response.json().await.unwrap();

                        format_about(config.github, user_info, user_stars)
                    } else {
                        String::from(FETCH_GITHUB_ERROR)
                    }
                }
                Err(_) => String::from(FETCH_GITHUB_ERROR),
            }
        }
        None => String::from(READ_JSON_ERROR),
    }
}

pub fn get_contacts() -> &'static String {
    CONTACTS.get_or_init(|| match read_config() {
        Some(config) => format_contacts(config),
        _ => String::from(READ_JSON_ERROR),
    })
}

fn read_config() -> Option<Config> {
    let config_json = include_str!("config.json");

    CONFIG.get_or_init(|| match serde_json::from_str::<Config>(&config_json) {
        Ok(config) => Some(config),
        Err(_) => None,
    });

    CONFIG.get().cloned()?
}

// Formatting functions

fn format_about(username: String, info: UserInfo, stats: UserStats) -> String {
    let name = info.name.unwrap_or(String::from("-"));
    let bio = info.bio.unwrap_or(String::from("-"));
    let repos = info.public_repos;
    let stars = stats.stars;
    let forks = stats.forks;
    let company = info.company.unwrap_or(String::from("-"));
    let location = info.location.unwrap_or(String::from("-"));
    let followers = info.followers;
    let following = info.following;
    let created_on = &info.created_at[..10];

    format!(
        r#"<span class="info">{}</span>@<span class="info">github</span>
----------------------
<span class="info">Name:</span> {}
<span class="info">Bio:</span> {}
<span class="info">Repos:</span> {}
<span class="info">Stars:</span> {}
<span class="info">Forks:</span> {}
<span class="info">Company:</span> {}
<span class="info">Location:</span> {}
<span class="info">Followers:</span> {}
<span class="info">Following:</span> {}
<span class="info">Created on:</span> {}
{BLOCKS}"#,
        username,
        name,
        bio,
        repos,
        stars,
        forks,
        company,
        location,
        followers,
        following,
        created_on
    )
}

fn format_contacts(config: Config) -> String {
    let github = format!(
        r#"Contact info and links -

  <a href="https://github.com/{}" target="_blank" style="color:MediumSlateBlue;font-weight:500;">Github</a>: github.com/{}"#,
        config.github, config.github
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

    format!(
        "{}{}{}{}",
        github,
        email.unwrap_or_default(),
        linkedin.unwrap_or_default(),
        twitter.unwrap_or_default()
    )
}

pub const BLOCKS: &str = r#"
<span class="blocks logo">█<span><span class="blocks command">█<span><span class="blocks logo">█<span><span class="blocks command">█<span><span class="blocks logo">█<span><span class="blocks command">█<span><span class="blocks logo">█<span><span class="blocks command">█<span>
<span class="blocks title">█<span><span class="blocks">█<span><span class="blocks title">█<span><span class="blocks">█<span><span class="blocks title">█<span><span class="blocks">█<span><span class="blocks title">█<span><span class="blocks">█<span>
"#;
