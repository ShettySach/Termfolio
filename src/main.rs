use async_std::sync::{Arc, Mutex};
use leptos::ev::SubmitEvent;
use leptos::html::{Form, Input};
use leptos::{
    component, create_effect, create_node_ref, create_signal, spawn_local, view, For, IntoView,
    NodeRef, SignalGet, SignalUpdate,
};

fn main() {
    leptos::mount_to_body(|| view! { <Base/> });
}

#[component]
fn Base() -> impl IntoView {
    let (prompts, set_prompts) = create_signal(1);

    let add_prompt = move |_| {
        set_prompts.update(|prompts| {
            *prompts += 1;
        });
    };

    let prompt_list = move || (0..prompts.get()).collect::<Vec<_>>();

    view! {
        <div>
                <For
                    each = prompt_list
                    key = |&prompt| prompt
                    children = move |_| {
                        view! {
                            <Prompt on:submit=add_prompt/>
                        }
                    }
                />
        </div>
    }
}

#[component]
fn Prompt() -> impl IntoView {
    let (out, set_out) = create_signal(String::new());

    let input_element: NodeRef<Input> = create_node_ref();
    let form_element: NodeRef<Form> = create_node_ref();
    let mutex = Arc::new(Mutex::new(()));

    let on_submit = move |ev: SubmitEvent| {
        let mutex = Arc::clone(&mutex);
        ev.prevent_default();

        let value = input_element().unwrap().value();
        let mutex_clone = Arc::clone(&mutex);

        spawn_local(async move {
            let _lock = mutex_clone.lock().await;
            set_out(termfolio::Command::process(&value).await);
            form_element().unwrap().set_inert(true);
        });
    };

    create_effect(move |_| {
        if let Some(ref_input) = input_element.get() {
            let _ = ref_input.on_mount(|input| {
                let _ = input.focus();
            });
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

/*
let prev_commands = window_event_listener(keydown, move |ev: KeyboardEvent| {
    let word = "h";

    if ev.key() == "ArrowUp" {
        input_element().unwrap().set_value(word);
    }
});
on_cleanup(move || prev_commands.remove());
*/
