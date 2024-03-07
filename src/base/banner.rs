use crate::commands::banner;
use leptos::{component, view, IntoView};

#[component]
pub fn Banner() -> impl IntoView {
    let banner = banner();

    view! {
        <p class="inline">"user@termfolio:~$ "</p>
        <p style="display:inline;padding:2px;">"help"</p>
        <pre>
            <div class="output" inner_html={banner}></div>
        </pre>
    }
}
