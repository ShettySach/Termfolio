use super::{About, Account, Links, Repos};
use std::collections::HashMap;

// Ascii art used for Github
const NEOFETCH: &str = include_str!("../../../configs/neofetch.txt");

// Language icons for repos
const RUST: &str = include_str!("../../../configs/lang_icons/ferris.txt");
const PYTHON: &str = include_str!("../../../configs/lang_icons/pythons.txt");
const JAVASCRIPT: &str = include_str!("../../../configs/lang_icons/javascript.txt");
const PLACEHOLDER: &str = include_str!("../../../configs/lang_icons/octocat.txt");

pub trait Formatter {
    fn formatter(self) -> String;
}

impl Formatter for About {
    fn formatter(self) -> String {
        let exp_string: String = self
            .experience
            .iter()
            .map(|exp| {
                format!(
                    r#"<span class="blu semibold">Title:</span> {}
<span class="blu semibold">Description:</span> 
{}"#,
                    exp.title,
                    exp.description
                        .iter()
                        .map(|s| format!(r#"<span class="blu semibold">*</span> {}"#, s))
                        .collect::<Vec<_>>()
                        .join("\n"),
                )
            })
            .collect::<Vec<String>>()
            .join("\n");

        let edu_string: String = self
            .education
            .iter()
            .map(|edu| {
                format!(
                    r#"<span class="blu semibold">Institute: </span>{}
<span class="blu semibold">Course:</span> {}
<span class="blu semibold">Duration:</span> {}
"#,
                    edu.institute, edu.course, edu.duration
                )
            })
            .collect::<Vec<String>>()
            .join("\n");

        let text = format!(
            r#"<center class="grn semibold">{}</center>
{}

<u class="rd semibold">Interests</u>

{}

<u class="rd semibold">Languages</u>

{}

<u class="rd semibold">Experience</u>

{}

<u class="rd semibold">Education</u>

{}
"#,
            self.name.to_uppercase(),
            self.intro,
            self.interests
                .iter()
                .map(|s| format!(r#"<span class="rd semibold">*</span> {}"#, s))
                .collect::<Vec<_>>()
                .join("\n"),
            format_langs(self.langs),
            exp_string,
            edu_string
        );

        format!(
            r#"


<div class="row" style="display: flex; flex-direction: row; align-items: center; justify-content: center;"> 
<div class="acols">{}</div>
</div>
"#,
            text
        )
    }
}

impl Formatter for Account {
    fn formatter(self) -> String {
        let name = self.info.name.unwrap_or(String::from("-"));
        let bio = self.info.bio.unwrap_or(String::from("-"));
        let repos = self.info.public_repos;
        let stars = self.stats.stars;
        let forks = self.stats.forks;
        let company = self.info.company.unwrap_or(String::from("-"));
        let location = self.info.location.unwrap_or(String::from("-"));
        let followers = self.info.followers;
        let following = self.info.following;
        let created_on = &self.info.created_at[..10];

        let text = format!(
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
            self.username,
            self.username,
            name,
            bio,
            repos,
            format_langs(self.langs),
            stars,
            forks,
            company,
            location,
            followers,
            following,
            created_on
        );

        format!(
            r#"<div class="row">
<div class="gcols">{}</div>
<div class="gcols">{}</div>
</div>"#,
            NEOFETCH, text
        )
    }
}

impl Formatter for Repos {
    fn formatter(self) -> String {
        let res: Vec<String> = self
            .repos
            .iter()
            .map(|repo| {
                let text = format!(
                    r#"<a href="{}" target="_blank" class="blu semibold">{}</a>

<span class="rd semibold">Description:</span> {}
<span class="rd semibold">Language:</span> <span style="color:{}">{}</span>
<span class="rd semibold">Stars:</span> <span class="ylw">{}</span>
<span class="rd semibold">Forks:</span> <span class="ylw">{}</span>

        "#,
                    repo.repo,
                    repo.name,
                    repo.description,
                    repo.language.color,
                    repo.language.name,
                    repo.stars,
                    repo.forks
                );

                format!(
                    r#"<div class="row">
<div class="rcols">{}</div>
<div class="rcols" style="width: 45%;">{}</div>
</div>"#,
                    lang_icon(&repo.language.name),
                    text
                )
            })
            .collect();

        let all_link = format!(
            r#"<a href="https://www.github.com/{}?tab=repositories" target="_blank" class="blu semibold">All repos</a>

<span class="rd semibold">Description:</span> All my Github repositories."#,
            self.username
        );

        let all = format!(
            r#"<div class="row">
<div class="rcols">{}</div>
<div class="rcols" style="width: 45%;">{}</div>
</div>"#,
            PLACEHOLDER, all_link
        );

        format!("{}\n{}", res.join("\n"), all)
    }
}

impl Formatter for Links {
    fn formatter(self) -> String {
        let mut result = String::new();

        result += &format!(
            r#"  <a href="https://github.com/{}" target="_blank" class="semibold"  style="color:var(--purple);">Github</a>: github.com/{}
"#,
            self.github, self.github
        );

        if let Some(email) = &self.email {
            result += &format!(
                r#"
  <a href="mailto:{}" target="_blank" class="semibold" style="color:var(--orange);">Email</a>: {}
  "#,
                email, email
            );
        }

        if let Some(linkedin) = &self.linkedin {
            result += &format!(
                r#"
  <a href="https://www.linkedin.com/{}" target="_blank" class="semibold" style="color:var(--dblue);">LinkedIn</a>: linkedin.com/{}
  "#,
                linkedin, linkedin
            );
        }

        if let Some(twitter) = &self.twitter {
            result += &format!(
                r#"
  <a href="https://www.twitter.com/{}" target="_blank" class="blu semibold">Twitter/X</a>: @{}
  "#,
                twitter, twitter
            );
        }

        result
    }
}

pub fn format_langs(langs: Vec<String>) -> String {
    let color_map: HashMap<&str, &str> = [
        ("Rust", "orange"),
        ("Python", "blue"),
        ("C", "dblue"),
        ("C++", "dblue"),
        ("Java", "red"),
        ("Haskell", "purple"),
        ("Zig", "orange"),
        ("Go", "blue"),
        ("Dart", "dblue"),
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

fn lang_icon(lang: &str) -> &str {
    match lang {
        "Rust" => RUST,
        "Python" | "Jupyter Notebook" => PYTHON,
        "CSS" | "HTML" | "JavaScript" => JAVASCRIPT,
        _ => PLACEHOLDER,
    }
}

const BLOCKS: &str = r#"<span class="blocks" style="color:var(--black)">█</span><span class="rd blocks">█</span><span class="grn blocks">█</span><span class="ylw blocks">█</span><span class="blu blocks">█</span><span class="blocks" style="color:var(--orange)">█</span><span class="blocks" style="color:var(--purple)">█</span><span class="blocks">█</span>"#;
