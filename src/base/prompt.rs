use leptos::ev::SubmitEvent;
use leptos::html::{Form, Input};
use leptos::{
    component, create_effect, create_node_ref, create_signal, spawn_local, view, IntoView, NodeRef,
    ReadSignal, WriteSignal,
};
use leptos_use::{
    use_color_mode_with_options, use_cycle_list_with_options, ColorMode, UseColorModeOptions,
    UseColorModeReturn, UseCycleListOptions, UseCycleListReturn,
};
use std::collections::VecDeque;

mod general;
use general::general_commands;
mod keyboard;
use keyboard::keyboard_commands;

#[component]
pub fn Prompt(
    submitter: WriteSignal<u8>,
    updater: WriteSignal<VecDeque<String>>,
    history: ReadSignal<VecDeque<String>>,
) -> impl IntoView {
    //Output and history index signals
    let (out, set_out) = create_signal(String::new());
    let (history_index, set_history_index): (ReadSignal<u8>, WriteSignal<u8>) = create_signal(0);

    //Form and input elements
    let form_element: NodeRef<Form> = create_node_ref();
    let input_element: NodeRef<Input> = create_node_ref();

    // Focus on the new prompt on mount
    create_effect(move |_| {
        if let Some(ref_input) = input_element.get() {
            let _ = ref_input.on_mount(|input| {
                let _ = input.focus();
            });
        }
    });

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
        let input_value = input_element().unwrap().value();
        let next = next.clone();

        spawn_local(async move {
            general_commands(
                input_value,
                state,
                next,
                set_out,
                submitter,
                updater,
                history,
            )
            .await
        });

        form_element().unwrap().set_inert(true);
        input_element().unwrap().set_inert(true);
    };

    // Event listener for Up and Down arrow keys, Tab and Ctrl/Command + L
    keyboard_commands(
        input_element,
        history,
        history_index,
        set_history_index,
        submitter,
    );

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
