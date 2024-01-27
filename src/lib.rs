mod fetch;
mod texts;

pub enum Command {
    Help,
    About,
    Repos,
    Experience,
    Links,
    Credits,
    Bash(Bash),
}

impl Command {
    fn from(inp: &str) -> Self {
        let inp = inp.trim();
        let inp = inp.split_once(' ').unwrap_or((inp, " "));

        match inp.0 {
            "help" | "h" | "termfolio" => Self::Help,
            "about" | "a" | "neofetch" => Self::About,
            "repos" | "r" | "projects" => Self::Repos,
            "experience" | "exp" | "xp" => Self::Experience,
            "links" | "l" => Self::Links,
            "credits" | "c" | "C" => Self::Credits,
            _ => Command::Bash(Bash::from(inp.0, inp.1)),
        }
    }

    async fn printout(&self) -> String {
        match self {
            Self::Help => format!("{}{}", texts::LOGO_V2, texts::HELP),
            Self::About => fetch::get_about().await,
            Self::Repos => fetch::get_repos().await,
            Self::Experience => String::from("Help"),
            Self::Links => fetch::get_contacts().to_string(),
            Self::Credits => format!("{}{}", texts::LOGO_V1, texts::CREDITS),
            Self::Bash(bash) => Bash::printout(&bash),
        }
    }

    pub async fn process(inp: &str) -> String {
        let command = Self::from(inp);
        Self::printout(&command).await
    }
}

pub enum Bash {
    Go,
    Create,
    Destroy,
    Cpy,
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
            "cp" => Self::Cpy,
            "mv" => Self::Move,
            "ls" | "cat" => Self::Show,
            "grep" | "which" | "find" => Self::Search,
            "pwd" => Self::Where,
            "nano" | "vim" | "nvim" | "emacs" | "hx" => Self::Edit,
            "sudo" | "chmod" => Self::Power,
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
            Self::Cpy => String::from("Nowhere to duplicate."),
            Self::Move => String::from("Nowhere to move."),
            Self::Show => String::from("Nothing to see."),
            Self::Search => String::from("Nowhere to search."),
            Self::Where => String::from("You are here."),
            Self::Edit => String::from("Nothing to make or change."),
            Self::Power => String::from("No power."),
            Self::You => String::from("Despite everything, it's still you."),
            Self::Echo(s) => String::from(s).replace("<", "‹").replace(">", "›"),
            Self::Nothing => String::new(),
            _ => String::from("Command not found..."),
        }
    }
}
