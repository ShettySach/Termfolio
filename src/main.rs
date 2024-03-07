use leptos::view;

mod base;
mod commands;
use base::Base;

fn main() {
    leptos::mount_to_body(|| view! { <Base/> });
}
