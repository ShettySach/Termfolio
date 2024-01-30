use serde::{Deserialize, Serialize};
use std::sync::OnceLock;
use tokio::sync::OnceCell;
use tokio::try_join;

mod formats;
use crate::texts::{FETCH_GITHUB_ERROR, READ_JSON_ERROR};
use formats::*;

// Config
const JSON: &str = include_str!("../configs/config.json");

// Once statics

static CONFIG: OnceLock<Option<Config>> = OnceLock::new();
static GITHUB: OnceCell<String> = OnceCell::const_new();
static REPOS: OnceCell<String> = OnceCell::const_new();
static CONTACTS: OnceLock<String> = OnceLock::new();

// Structs

#[derive(Deserialize, Serialize, Clone)]
pub struct Config {
    pub github: String,
    pub about: About,
    pub links: Links,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct About {
    pub name: String,
    pub intro: String,
    pub interests: Vec<String>,
    pub langs: Vec<String>,
    pub experience: Vec<Experience>,
    pub education: Vec<Education>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Experience {
    pub title: String,
    pub description: Vec<String>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Education {
    pub institute: String,
    pub course: String,
    pub duration: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Links {
    pub email: Option<String>,
    pub linkedin: Option<String>,
    pub twitter: Option<String>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct UserInfo {
    pub name: Option<String>,
    pub bio: Option<String>,
    pub public_repos: u32,
    pub company: Option<String>,
    pub location: Option<String>,
    pub followers: u32,
    pub following: u32,
    pub created_at: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct UserStats {
    pub stars: u32,
    pub forks: u32,
}

#[derive(Deserialize, Serialize)]
pub struct ApiResponse {
    pub response: Vec<Repository>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Repository {
    pub name: String,
    pub repo: String,
    pub description: String,
    pub language: Language,
    pub stars: u32,
    pub forks: u32,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Language {
    pub name: String,
    pub color: String,
}

// Once Functions

fn read_config() -> Option<Config> {
    CONFIG.get_or_init(|| match serde_json::from_str::<Config>(&JSON) {
        Ok(config) => Some(config),
        Err(_) => None,
    });

    CONFIG.get().cloned()?
}

pub fn get_about() -> String {
    match read_config() {
        Some(config) => format_about(config.about),
        _ => String::from(READ_JSON_ERROR),
    }
}

pub async fn get_github() -> String {
    GITHUB.get_or_init(|| fetch_github()).await;

    match GITHUB.get() {
        Some(user) => user.to_string(),
        _ => String::from(FETCH_GITHUB_ERROR),
    }
}

pub async fn get_repos() -> String {
    REPOS.get_or_init(|| fetch_repos()).await;

    match REPOS.get() {
        Some(repos) => repos.to_string(),
        _ => String::from(FETCH_GITHUB_ERROR),
    }
}

pub fn get_contacts() -> &'static String {
    CONTACTS.get_or_init(|| match read_config() {
        Some(config) => format_contacts(config),
        _ => String::from(READ_JSON_ERROR),
    })
}

// Fetch functions

async fn fetch_github() -> String {
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

                        format_github(config.github, config.about.langs, user_info, user_stats)
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

async fn fetch_repos() -> String {
    match read_config() {
        Some(config) => {
            let repos_url = format!(
                "https://gh-pinned-repos-api.ysnirix.xyz/api/get/?username={}",
                config.github
            );

            match reqwest::get(&repos_url).await {
                Ok(response) => {
                    let repos: ApiResponse = response.json().await.unwrap();
                    format_repos(config.github, repos.response)
                }
                Err(_) => String::from(FETCH_GITHUB_ERROR),
            }
        }
        None => String::from(READ_JSON_ERROR),
    }
}
