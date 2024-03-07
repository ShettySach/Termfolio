use crate::commands::autocomplete;
use leptos::ev::{keydown, KeyboardEvent};
use leptos::html::Input;
use leptos::{NodeRef, ReadSignal, SignalGetUntracked, SignalUpdate, WriteSignal};
use leptos_use::use_event_listener;
use std::collections::VecDeque;

pub fn keyboard_commands(
    input_element: NodeRef<Input>,
    history: ReadSignal<VecDeque<String>>,
    history_index: ReadSignal<u8>,
    set_history_index: WriteSignal<u8>,
    submitter: WriteSignal<u8>,
) {
    let _ = use_event_listener(input_element, keydown, move |ev: KeyboardEvent| {
        let index = history_index.get_untracked().into();
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
                } else if index == 1 {
                    inp.set_value("");
                    set_history_index.update(|history_index| *history_index -= 1);
                }
            }
            //Autocomplete
            "Tab" => {
                ev.prevent_default();
                inp.set_value(autocomplete(&inp.value()));
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
}
