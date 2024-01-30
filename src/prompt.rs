use leptos::ev::{keydown, KeyboardEvent, SubmitEvent};
use leptos::html::{Form, Input};
use leptos::{
    component, create_effect, create_node_ref, create_signal, spawn_local, view, IntoView, NodeRef,
    ReadSignal, SignalGetUntracked, SignalUpdate, WriteSignal,
};
use leptos_use::use_event_listener;
use std::collections::VecDeque;

#[component]
pub fn Prompt(
    submitter: WriteSignal<u32>,
    updater: WriteSignal<VecDeque<String>>,
    history: ReadSignal<VecDeque<String>>,
) -> impl IntoView {
    let (out, set_out) = create_signal(String::new());
    let (history_index, set_history_index) = create_signal(0);

    let input_element: NodeRef<Input> = create_node_ref();
    let form_element: NodeRef<Form> = create_node_ref();

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        let value = input_element().unwrap().value();

        spawn_local(async move {
            match &value[..] {
                "clear" => {
                    submitter.update(|prompts| {
                        *prompts = 0;
                    });
                }
                "history" => {
                    let hist: Vec<String> = history.get_untracked().into();
                    let hist: Vec<String> = hist
                        .iter()
                        .rev()
                        .enumerate()
                        .map(|(i, c)| format!("{} {}", i + 1, c))
                        .collect();
                    set_out(hist.join("\n"));
                }
                _ => set_out(termfolio::Command::process(&value).await),
            }

            updater.update(|hist| {
                if value != "" {
                    let value = value.replace("<", "‹").replace(">", "›");
                    hist.push_front(value);
                    if hist.len() > 15 {
                        hist.pop_back();
                    }
                }
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
        let hist = history.get_untracked();
        let inp = input_element.get_untracked().unwrap();

        match &ev.key()[..] {
            "ArrowUp" => {
                if index < hist.len() {
                    inp.set_value(&hist[index]);
                    set_history_index.update(|history_index| *history_index += 1);
                }
            }
            "ArrowDown" => {
                if index > 1 {
                    inp.set_value(&hist[index - 2]);
                    set_history_index.update(|history_index| *history_index -= 1);
                } else if index != 0 {
                    inp.set_value("");
                    set_history_index.update(|history_index| *history_index -= 1);
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
