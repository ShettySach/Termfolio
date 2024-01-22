use leptos::ev::{keydown, KeyboardEvent, SubmitEvent};
use leptos::html::{Form, Input};
use leptos::on_cleanup;
use leptos::{
    component, create_effect, create_node_ref, create_signal, view, window_event_listener, For,
    IntoView, NodeRef, SignalGet, SignalUpdate,
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
                    each=prompt_list
                    key=|&prompt| prompt
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

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();

        let value = input_element().unwrap().value();
        input_element().unwrap().set_value(&value);
        set_out(termfolio::Command::process(&value));

        form_element().unwrap().set_inert(true);
    };

    create_effect(move |_| {
        if let Some(ref_input) = input_element.get() {
            let _ = ref_input.on_mount(|input| {
                let _ = input.focus();
            });
        }
    });

    let prev_command = window_event_listener(keydown, move |ev: KeyboardEvent| {
        let word = "testing";

        if ev.key() == "ArrowUp" {
            input_element().unwrap().set_value(word);
        } else if ev.key() == "ArrowDown" {
            input_element().unwrap().set_value(word);
        }
    });
    on_cleanup(move || prev_command.remove());

    view! {
        <form
            on:submit=on_submit node_ref=form_element>
            <p class="inline">"user@termfolio:~$ "</p>
            <input type="text" maxlength=42 value=out node_ref=input_element/>
        </form>
        <pre>
            <p class="output" inner_html={out}></p>
        </pre>
    }
}
