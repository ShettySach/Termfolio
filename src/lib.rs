mod texts;

pub enum Command {
    Help,
    About,
    Projects,
    Experience,
    Contact,
    Others,
    Bash(Bash),
}

impl Command {
    fn from(inp: &str) -> Self {
        let inp = inp.trim();
        let inp = inp.split_once(' ').unwrap_or((inp, " "));

        match inp.0 {
            "help" | "h" => Self::Help,
            "about" | "a" | "neofetch" => Self::About,
            "projects" | "projs" | "p" => Self::Projects,
            "experience" | "exp" | "xp" => Self::Experience,
            "contacts" | "c" => Self::Contact,
            "others" | "o" => Self::Others,
            _ => Command::Bash(Bash::from(inp.0, inp.1)),
        }
    }

    fn printout(&self) -> String {
        match self {
            Self::Help => String::from(texts::HELP),
            Self::About => String::from("Help"),
            Self::Projects => String::from("Help"),
            Self::Experience => String::from("Help"),
            Self::Contact => String::from(texts::CONTACTS),
            Self::Others => String::from("Others"),
            Self::Bash(bash) => Bash::printout(&bash),
        }
    }

    pub fn process(inp: &str) -> String {
        let command = Self::from(inp);
        Self::printout(&command)
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
            "echo" => Self::Echo(inp1.to_string()),
            "" => Self::Nothing,
            _ => Self::Invalid,
        }
    }

    pub fn printout(&self) -> String {
        match self {
            Self::Go => String::from("Nowhere to go"),
            Self::Create => String::from("Nowhere to create"),
            Self::Destroy => String::from("Nothing to destroy"),
            Self::Cpy => String::from("Nowhere to duplicate"),
            Self::Move => String::from("Nowhere to move"),
            Self::Show => String::from("Nothing to see"),
            Self::Search => String::from("Nowhere to search"),
            Self::Where => String::from("You are here"),
            Self::Edit => String::from("Nothing to make or change"),
            Self::Power => String::from("No power"),
            Self::Echo(s) => String::from(s).replace("<", "‹").replace(">", "›"),
            Self::Nothing => String::new(),
            _ => String::from("Command not found..."),
        }
    }
}
