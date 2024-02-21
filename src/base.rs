use leptos::{component, create_signal, view, For, IntoView, SignalGet};
use std::collections::VecDeque;

mod banner;
use banner::Banner;
mod prompt;
use prompt::Prompt;

#[component]
pub fn Base() -> impl IntoView {
    // Signals for number of prompts and history vector
    let (prompts, set_prompts) = create_signal(1);
    let (history, set_history) = create_signal(VecDeque::new());

    let prompt_list = move || (0..prompts.get()).collect::<Vec<u8>>();

    view! {
        <div>
            <Banner/>
            <For
                each = prompt_list
                key = |&prompt| prompt
                children = move |_| {
                    view! {
                        <Prompt
                            submitter=set_prompts
                            updater=set_history
                            history=history/>
                    }
                }
            />
        </div>
    }
}
