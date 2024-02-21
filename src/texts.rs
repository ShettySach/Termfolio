pub const HELP: &str = r#"<span class="grn semibold">  ________________  __  _____________  __    ________ 
 /_  __/ ____/ __ \/  |/  / ____/ __ \/ /   /  _/ __ \
  / / / __/ / /_/ / /|_/ / /_  / / / / /    / // / / /
 / / / /___/ _, _/ /  / / __/ / /_/ / /____/ // /_/ / 
/_/ /_____/_/ |_/_/  /_/_/    \____/_____/___/\____/  
</span>

Hello, welcome to <u class="blu semibold">Termfolio</u>. Type one of these commands -

  <span class="rd semibold">about</span> - View about me
  <span class="rd semibold">github</span> - View about Github profile 
  <span class="rd semibold">repos</span> - View about my pinned repos / projects
  <span class="rd semibold">links</span> - View contact info and links
  <span class="rd semibold">help</span> - View this help section
  <span class="rd semibold">theme</span> - Cycle through themes
  <span class="rd semibold">credits</span> - View credits and repo"#;

pub const CREDITS: &str = r#"<span class="grn"> _____ ______________  _________ _____ _     _____ _____ 
|_   _|  ___| ___ \  \/  ||  ___|  _  | |   |_   _|  _  |
  | | | |__ | |_/ / .  . || |_  | | | | |     | | | | | |
  | | |  __||    /| |\/| ||  _| | | | | |     | | | | | |
  | | | |___| |\ \| |  | || |   \ \_/ / |_____| |_\ \_/ /
  \_/ \____/\_| \_\_|  |_/\_|    \___/\_____/\___/ \___/ 
</span>
Terminal style portfolio website, made in Leptos, Rust.
Made by <span class="rd semibold">Sachith C Shetty</span> 
  
  <a href="https://github.com/shettysach" target="_blank" class="blu semibold">Github</a>: github.com/shettysach

  <a href="https://github.com/shettysach/termfolio" target="_blank" class="blu semibold">Repo</a>: github.com/shettysach/termfolio

<span class="rd semibold">APIs used -</span>

* <a 
    href="https://docs.github.com/en/rest/about-the-rest-api"
    target="_blank"
    class="blu semibold">Github REST API</a>

* <a 
    href="https://github.com/Ysn4Irix/gh-pinned-repos-api"
    target="_blank" 
    class="blu semibold">Pinned repos</a> - Ysn4Irix/gh-pinned-repos-api

* <a 
    href="https://github.com/idealclover/GitHub-Star-Counter"
    target="_blank"
    class="blu semibold">Total stars and forks</a> - idealclover/GitHub-Star-Counter"#;

pub const READ_JSON_ERROR: &str = r#"<span class="rd semibold">Error reading config.json</span>"#;
pub const FETCH_GITHUB_ERROR: &str =
    r#"<span class="rd semibold">Error fetching data from Github.</span>"#;
