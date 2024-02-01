use leptos::view;

mod base;
use base::Base;

fn main() {
    leptos::mount_to_body(|| view! { <Base/> });
}
