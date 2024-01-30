use leptos::{component, create_signal, view, For, IntoView, SignalGet};
use std::collections::VecDeque;

mod prompt;
use prompt::Prompt;

fn main() {
    leptos::mount_to_body(|| view! { <Base/> });
}

#[component]
fn Base() -> impl IntoView {
    let (prompts, set_prompts) = create_signal(1);
    let (history, set_history) = create_signal(VecDeque::new());

    let prompt_list = move || (0..prompts.get()).collect::<Vec<_>>();

    view! {
        <div>
            <For
                each = prompt_list
                key = |&prompt| prompt
                children = move |_| {
                    view! {
                        <Prompt submitter=set_prompts updater=set_history history=history/>
                    }
                }
            />
        </div>
    }
}
