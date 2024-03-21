use serde::{Deserialize, Serialize};

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
    pub github: String,
    pub email: Option<String>,
    pub linkedin: Option<String>,
    pub twitter: Option<String>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Account {
    pub username: String,
    pub langs: Vec<String>,
    pub info: UserInfo,
    pub stats: UserStats,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct UserInfo {
    pub name: Option<String>,
    pub bio: Option<String>,
    pub public_repos: u16,
    pub company: Option<String>,
    pub location: Option<String>,
    pub followers: u16,
    pub following: u16,
    pub created_at: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct UserStats {
    pub stars: u16,
    pub forks: u16,
}

#[derive(Deserialize, Serialize)]
pub struct ApiResponse {
    pub response: Vec<Repository>,
}

#[derive(Deserialize, Serialize)]
pub struct Repos {
    pub username: String,
    pub repos: Vec<Repository>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Repository {
    pub name: String,
    pub repo: String,
    pub description: String,
    pub language: Language,
    pub stars: u16,
    pub forks: u16,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Language {
    pub name: String,
    pub color: String,
}
