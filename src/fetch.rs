use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::OnceLock;
use tokio::sync::OnceCell;
use tokio::try_join;

// Error lines

pub const READ_JSON_ERROR: &str = r#"<span class="rd semibold">Error reading config.json</span>"#;
pub const FETCH_GITHUB_ERROR: &str =
    r#"<span class="rd semibold">Error fetching data from Github.</span>"#;

// Once statics

static CONFIG: OnceLock<Option<Config>> = OnceLock::new();
static ABOUT: OnceCell<String> = OnceCell::const_new();
static REPOS: OnceCell<String> = OnceCell::const_new();
static CONTACTS: OnceLock<String> = OnceLock::new();

// Structs

#[derive(Deserialize, Serialize, Clone)]
struct Config {
    github: String,
    email: Option<String>,
    linkedin: Option<String>,
    twitter: Option<String>,
    langs: Vec<String>,
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

#[derive(Deserialize, Serialize, Clone)]
struct Repo {
    name: String,
    link: String,
    description: String,
    language: String,
    language_color: String,
    stars: String,
    forks: String,
}
// Functions

fn read_config() -> Option<Config> {
    let config_json = include_str!("config.json");

    CONFIG.get_or_init(|| match serde_json::from_str::<Config>(&config_json) {
        Ok(config) => Some(config),
        Err(_) => None,
    });

    CONFIG.get().cloned()?
}

pub async fn get_about() -> String {
    ABOUT.get_or_init(|| fetch_about()).await;

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

async fn fetch_about() -> String {
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
                        let user_stats: UserStats = stats_response.json().await.unwrap();

                        format_about(config.github, config.langs, user_info, user_stats)
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

pub async fn get_repos() -> String {
    REPOS.get_or_init(|| read_repos()).await;

    match REPOS.get() {
        Some(repos) => repos.to_string(),
        _ => String::from(FETCH_GITHUB_ERROR),
    }
}

#[allow(dead_code)]
async fn read_repos() -> String {
    match read_config() {
        Some(_config) => {
            let response = include_str!("repos.txt");
            let repos: Vec<Repo> = serde_json::from_str(&response).unwrap();
            format_repos(repos)
        }
        None => String::from(READ_JSON_ERROR),
    }
}

async fn fetch_repos() -> String {
    match read_config() {
        Some(config) => {
            let repos_url = format!("https://pinned.thrzl.xyz/{}", config.github);
            let response = reqwest::get(&repos_url)
                .await
                .unwrap()
                .text()
                .await
                .unwrap();
            let _repos: Vec<Repo> = serde_json::from_str(&response).unwrap();
            // format_repos(repos)
            response
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

//https://pinned.thrzl.xyz/shettysach

// Formatting functions

fn format_about(username: String, langs: Vec<String>, info: UserInfo, stats: UserStats) -> String {
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
        r#"<a href="https://www.github.com/{}" style="text-decoration:none" target="_blank"><span class="grn semibold">{}</span>@<span class="grn semibold">github</span></a>
----------------------
<span class="grn semibold">Name:</span> {}
<span class="grn semibold">Bio:</span> {}
<span class="grn semibold">Repos:</span> {}
<span class="grn semibold">Langs:</span> {}
<span class="grn semibold">Stars:</span> {}
<span class="grn semibold">Forks:</span> {}
<span class="grn semibold">Company:</span> {}
<span class="grn semibold">Location:</span> {}
<span class="grn semibold">Followers:</span> {}
<span class="grn semibold">Following:</span> {}
<span class="grn semibold">Created on:</span> {}

{BLOCKS}"#,
        username,
        username,
        name,
        bio,
        repos,
        format_langs(langs),
        stars,
        forks,
        company,
        location,
        followers,
        following,
        created_on
    )
}

fn format_repos(repos: Vec<Repo>) -> String {
    let res: Vec<String> = repos
        .iter()
        .map(|repo| {
            format!(
                r#"<a href="{}" target="_blank" class="rd semibold">{}</a>
  <span class="blu semibold">Description:</span> {}
  <span class="blu semibold">Language:</span> <span style="color:{}">{}</span>
  <span class="blu semibold">Stars:</span> {}
  <span class="blu semibold">Forks:</span> {}
        "#,
                repo.link,
                repo.name,
                repo.description,
                repo.language_color,
                repo.language,
                repo.stars,
                repo.forks
            )
        })
        .collect();

    res.join("\n")
}

fn format_contacts(config: Config) -> String {
    let github = format!(
        r#"Contact info and links -

  <a href="https://github.com/{}" target="_blank" style="color:var(--purple);font-weight:500;">Github</a>: github.com/{}"#,
        config.github, config.github
    );

    let email = config.email.map(|email| {
        format!(
            r#"
  <a href="mailto:{}" target="_blank" style="color:var(--orange);font-weight:500;">Email</a>: {}"#,
            email, email
        )
    });

    let linkedin = config.linkedin.map(|linkedin| {
                    format!(
            r#"
  <a href="https://www.linkedin.com/{}" target="_blank" style="color:var(--dblue);font-weight:500;">LinkedIn</a>: linkedin.com/{}"#,
            linkedin, linkedin
                    )
                });

    let twitter = config.twitter.map(|twitter| {
                    format!(
            r#"
  <a href="https://www.twitter.com/{}" target="_blank" style="color:(--blue);font-weight:500;">Twitter/X</a>: @{}"#,
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

fn format_langs(langs: Vec<String>) -> String {
    let color_map: HashMap<&str, &str> = [
        ("Rust", "orange"),
        ("Python", "blue"),
        ("C", "dblue"),
        ("C++", "dblue"),
        ("Java", "red"),
        ("Haskell", "purple"),
        ("Zig", "orange"),
        ("Go", "blue"),
        ("JavaScript", "yellow"),
        ("TypeScript", "blue"),
        ("Bash", "dgreen"),
    ]
    .into();

    let formatted_langs: Vec<String> = langs
        .into_iter()
        .map(|lang| {
            color_map.get(lang.as_str()).map_or_else(
                || format!(r#"<span>{}</span>"#, lang),
                |color| format!(r#"<span style="color:var(--{});">{}</span>"#, color, lang),
            )
        })
        .collect();

    formatted_langs.join(" ")
}

const BLOCKS: &str = r#"<span class="blocks" style="color:var(--black)">█</span><span class="rd blocks">█</span><span class="grn blocks">█</span><span class="ylw blocks">█</span><span class="blu blocks">█</span><span class="blocks" style="color:var(--orange)">█</span><span class="blocks" style="color:var(--purple)">█</span><span class="blocks">█</span>"#;
