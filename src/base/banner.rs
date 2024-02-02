use leptos::{component, view, IntoView};

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

#[component]
pub fn Banner() -> impl IntoView {
    view! {
        <p class="inline">"user@termfolio:~$ "</p>
        <p style="display:inline;padding:2px;">"help"</p>
        <pre>
            <div class="output" inner_html={HELP}></div>
        </pre>
    }
}
