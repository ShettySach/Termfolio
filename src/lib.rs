mod fetch;
mod texts;

pub enum Command {
    Help,
    About,
    Github,
    Repos,
    Links,
    Credits,
    Bash(Bash),
}

impl Command {
    fn from(inp0: &str, inp1: &str) -> Self {
        match inp0 {
            "help" | "termfolio" => Self::Help,
            "about" => Self::About,
            "github" | "neofetch" => Self::Github,
            "repos" | "projects" => Self::Repos,
            "links" => Self::Links,
            "credits" => Self::Credits,
            _ => Command::Bash(Bash::from(inp0, inp1)),
        }
    }

    async fn printout(&self) -> String {
        match self {
            Self::Help => format!("{}{}", texts::LOGO_V2, texts::HELP),
            Self::About => fetch::get_about(),
            Self::Github => fetch::get_github().await,
            Self::Repos => fetch::get_repos().await,
            Self::Links => fetch::get_contacts().to_string(),
            Self::Credits => format!("{}{}", texts::LOGO_V1, texts::CREDITS),
            Self::Bash(bash) => Bash::printout(&bash),
        }
    }

    pub async fn process(inp0: &str, inp1: &str) -> String {
        let command = Self::from(inp0, inp1);
        Self::printout(&command).await
    }
}

pub enum Bash {
    Go,
    Create,
    Destroy,
    Duplicate,
    Move,
    Show,
    Search,
    Where,
    Edit,
    Power,
    You,
    Echo(String),
    Nothing,
    Invalid,
}

impl Bash {
    pub fn from(inp0: &str, inp1: &str) -> Self {
        match inp0 {
            "cd" => Self::Go,
            "mkdir" | "touch" => Self::Create,
            "rm" | "rmdir" => Self::Destroy,
            "cp" => Self::Duplicate,
            "mv" => Self::Move,
            "ls" | "cat" => Self::Show,
            "grep" | "which" | "find" => Self::Search,
            "pwd" => Self::Where,
            "nano" | "vi" | "vim" | "nvim" | "emacs" | "hx" => Self::Edit,
            "su" | "sudo" | "chmod" => Self::Power,
            "whoami" => Self::You,
            "echo" => Self::Echo(String::from(inp1)),
            "" => Self::Nothing,
            _ => Self::Invalid,
        }
    }

    pub fn printout(&self) -> String {
        match self {
            Self::Go => String::from("Nowhere to go."),
            Self::Create => String::from("Nowhere to create."),
            Self::Destroy => String::from("Nothing to destroy."),
            Self::Duplicate => String::from("Nowhere to duplicate."),
            Self::Move => String::from("Nowhere to move."),
            Self::Show => String::from("Nothing to see."),
            Self::Search => String::from("Nowhere to search."),
            Self::Where => String::from("You are here."),
            Self::Edit => String::from("Nothing to change."),
            Self::Power => String::from("With great power comes great responsibility."),
            Self::You => String::from("Despite everything, it's still you."),
            Self::Echo(s) => String::from(s).replace("<", "‹").replace(">", "›"),
            Self::Nothing => String::new(),
            _ => String::from("Command not found..."),
        }
    }
}

pub fn autocomplete(inp: &str) -> &str {
    let inp = inp.trim();

    let comms = [
        "help", "about", "github", "repos", "links", "theme", "credits",
    ];

    for &c in comms.iter() {
        if c.starts_with(inp) {
            return c;
        }
    }

    inp
}
