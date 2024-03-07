use crate::commands::Command;
use leptos::{ReadSignal, Signal, SignalGetUntracked, SignalUpdate, WriteSignal};
use leptos_use::ColorMode;
use std::collections::VecDeque;

pub async fn general_commands<F>(
    value: String,
    state: Signal<ColorMode>,
    next: F,
    set_out: WriteSignal<String>,
    submitter: WriteSignal<u8>,
    updater: WriteSignal<VecDeque<String>>,
    history: ReadSignal<VecDeque<String>>,
) where
    F: Fn(),
{
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
        _ => set_out(Command::process(val.0, val.1).await),
    }

    updater.update(|hist| {
        if !value.is_empty() && hist.front() != Some(&value) {
            hist.push_front(value);
            if hist.len() > 20 {
                hist.pop_back();
            }
        }
    });

    // Clears if max limit is reached
    submitter.update(|prompts| {
        if *prompts == u8::MAX {
            *prompts = 0;
        }
    });

    submitter.update(|prompts| {
        *prompts += 1;
    });
}
