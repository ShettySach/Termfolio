use leptos::ev::{keydown, KeyboardEvent, SubmitEvent};
use leptos::html::{Form, Input};
use leptos::{
    component, create_effect, create_node_ref, create_signal, spawn_local, view, For, IntoView,
    NodeRef, SignalGet, SignalGetUntracked, SignalUpdate, WriteSignal,
};
use leptos_use::use_event_listener;
use std::collections::VecDeque;

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
                        <Prompt submitter=set_prompts updater=set_history history=history.get()/>
                    }
                }
            />
        </div>
    }
}

#[component]
fn Prompt(
    submitter: WriteSignal<u32>,
    updater: WriteSignal<VecDeque<String>>,
    history: VecDeque<String>,
) -> impl IntoView {
    let (out, set_out) = create_signal(String::new());
    let (history_index, set_history_index) = create_signal(0);

    let input_element: NodeRef<Input> = create_node_ref();
    let form_element: NodeRef<Form> = create_node_ref();

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        let value = input_element().unwrap().value();

        spawn_local(async move {
            set_out(termfolio::Command::process(&value).await);

            updater.update(|hist| {
                hist.push_front(value);
            });

            submitter.update(|prompts| {
                *prompts += 1;
            });
        });
    };

    create_effect(move |_| {
        if let Some(ref_input) = input_element.get() {
            let _ = ref_input.on_mount(|input| {
                let _ = input.focus();
            });
        }
    });

    let _ = use_event_listener(input_element, keydown, move |ev: KeyboardEvent| {
        let index = history_index.get_untracked();

        match &ev.key()[..] {
            "ArrowUp" => {
                if index < history.len() {
                    input_element().unwrap().set_value(&history[index]);
                    set_history_index.update(|history_index| *history_index += 1);
                }
            }
            "ArrowDown" => {
                if index > 1 {
                    input_element().unwrap().set_value(&history[index - 2]);
                    set_history_index.update(|history_index| *history_index -= 1);
                } else {
                    if input_element.get_untracked().unwrap().value() != "" {
                        set_history_index.update(|history_index| *history_index -= 1);
                        input_element.get_untracked().unwrap().set_value("");
                    }
                }
            }
            _ => {}
        }
    });

    view! {
        <form
            on:submit=on_submit node_ref=form_element>
            <p class="inline">"user@termfolio:~$ "</p>
            <input type="text" maxlength=42 value=out node_ref=input_element/>
        </form>
        <pre>
            <div class="output" inner_html={out}></div>
        </pre>
    }
}
