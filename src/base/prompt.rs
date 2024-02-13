use leptos::ev::{keydown, KeyboardEvent, SubmitEvent};
use leptos::html::{Form, Input};
use leptos::{
    component, create_effect, create_node_ref, create_signal, spawn_local, view, IntoView, NodeRef,
    ReadSignal, SignalGetUntracked, SignalUpdate, WriteSignal,
};
use leptos_use::{
    use_color_mode_with_options, use_cycle_list_with_options, use_event_listener, ColorMode,
    UseColorModeOptions, UseColorModeReturn, UseCycleListOptions, UseCycleListReturn,
};
use std::collections::VecDeque;

#[component]
pub fn Prompt(
    submitter: WriteSignal<u8>,
    updater: WriteSignal<VecDeque<String>>,
    history: ReadSignal<VecDeque<String>>,
) -> impl IntoView {
    //Output and history index signals
    let (out, set_out) = create_signal(String::new());
    let (history_index, set_history_index) = create_signal(0);

    //Form and input elements
    let form_element: NodeRef<Form> = create_node_ref();
    let input_element: NodeRef<Input> = create_node_ref();

    //Themes
    let UseColorModeReturn { mode, set_mode, .. } = use_color_mode_with_options(
        UseColorModeOptions::default()
            .custom_modes(vec!["catppuccin".into(), "nord".into(), "classic".into()])
            .initial_value(ColorMode::from("tokyo-night")),
    );

    let UseCycleListReturn { state, next, .. } = use_cycle_list_with_options(
        vec![
            ColorMode::Custom("catppuccin".into()),
            ColorMode::Custom("nord".into()),
            ColorMode::Custom("classic".into()),
            ColorMode::Custom("tokyo-night".into()),
        ],
        UseCycleListOptions::default().initial_value(Some((mode, set_mode).into())),
    );

    //On submit
    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        let value = input_element().unwrap().value();
        let next = next.clone();

        spawn_local(async move {
            let value = value.trim().replace("<", "‹").replace(">", "›");
            let val = value.split_once(' ').unwrap_or((&value, ""));

            match val.0 {
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
                "theme" | "t" | "wal" => {
                    next();
                    let new_theme = state.get_untracked();
                    set_out(format!(
                        r#"Theme changed to: <b class="grn">{new_theme}</b>"#
                    ));
                }
                _ => set_out(termfolio::Command::process(val.0, val.1).await),
            }

            updater.update(|hist| {
                if !value.is_empty() && hist.front() != Some(&value) {
                    hist.push_front(value);
                    if hist.len() > 20 {
                        hist.pop_back();
                    }
                }
            });

            submitter.update(|prompts| {
                if *prompts < u8::MAX {
                    *prompts += 1;
                }
            });
        });

        form_element().unwrap().set_inert(true);
        input_element().unwrap().set_inert(true);
    };

    // Focus on the new prompt on mount
    create_effect(move |_| {
        if let Some(ref_input) = input_element.get() {
            let _ = ref_input.on_mount(|input| {
                let _ = input.focus();
            });
        }
    });

    // Event listener for Up and Down arrow keys, Tab and Ctrl/Command + L
    let _ = use_event_listener(input_element, keydown, move |ev: KeyboardEvent| {
        let index = history_index.get_untracked();
        let hist = history.get_untracked();
        let inp = input_element.get_untracked().unwrap();

        match &ev.key()[..] {
            //Previous command in history
            "ArrowUp" => {
                ev.prevent_default();
                if index < hist.len() {
                    inp.set_value(&hist[index]);
                    set_history_index.update(|history_index| *history_index += 1);
                }
            }
            //Next command in history
            "ArrowDown" => {
                if index > 1 {
                    inp.set_value(&hist[index - 2]);
                    set_history_index.update(|history_index| *history_index -= 1);
                } else if index != 0 {
                    inp.set_value("");
                    set_history_index.update(|history_index| *history_index -= 1);
                }
            }
            //Autocomplete
            "Tab" => {
                ev.prevent_default();
                inp.set_value(termfolio::autocomplete(&inp.value()));
            }
            _ => {}
        }

        //Clear
        if (ev.ctrl_key() || ev.meta_key()) && (ev.key() == "l" || ev.key() == "L") {
            ev.prevent_default();
            submitter.update(|prompts| {
                *prompts = 0;
            });
            submitter.update(|prompts| {
                *prompts += 1;
            });
        }
    });

    view! {
        <form
            id="prompt-form"
            on:submit=on_submit node_ref=form_element>
            <p class="inline">"user@termfolio:~$ "</p>
            <input
                id="prompt-form" autocomplete="off"
                class="inp" type="text" maxlength=42 spellcheck="false"
                value=out node_ref=input_element/>
        </form>
        <pre>
            <div class="output" inner_html={out}></div>
        </pre>
    }
}
