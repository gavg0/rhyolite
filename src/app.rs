// use leptos::leptos_dom::ev::SubmitEvent;
use leptos::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

#[component]
pub fn App() -> impl IntoView {
    let (title_text, set_title_text) = create_signal(String::new());
    let (textbox_text, set_textbox_text) = create_signal(String::new());

    // let update_name = move |ev| {
    //     let v = event_target_value(&ev);
    //     set_name.set(v);
    // };

    let update_title = move |ev| {
        let v = event_target_value(&ev);
        set_title_text.set(v);
    };

    let update_textbox = move |ev| {
        let v = event_target_value(&ev);
        set_textbox_text.set(v);
    };



    // let greet = move |ev: SubmitEvent| {
    //     ev.prevent_default();
    //     spawn_local(async move {
    //         let name = name.get_untracked();
    //         if name.is_empty() {
    //             return;
    //         }

    //         let args = serde_wasm_bindgen::to_value(&GreetArgs { name: &name }).unwrap();
    //         // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    //         let new_msg = invoke("greet", args).await.as_string().unwrap();
    //         set_greet_msg.set(new_msg);
    //     });
    // };

    view! {
        <main class="container">
            <div class="title-container title-textarea">
                <textarea
                    class="rounded-container"
                    placeholder="Enter Title here..."
                    value=title_text
                    on:input=update_title
                />
            </div>
            <div class="textbox-container">
                <textarea
                    class="rounded-container"
                    placeholder="start typing..."
                    value=textbox_text
                    on:input=update_textbox
                />
            </div>

            // <form class="row" on:submit=greet>
            //     <input
            //         id="greet-input"
            //         placeholder="Enter a name..."
            //         on:input=update_name
            //     />
            //     <button type="submit">"Greet"</button>
            // </form>
            // <p>{ move || greet_msg.get() }</p>
        </main>
    }
}
