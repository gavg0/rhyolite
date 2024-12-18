use ev::KeyboardEvent;
// use leptos::leptos_dom::ev::SubmitEvent;
use leptos::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use std::time::Duration;
use serde_json::json;
use uuid;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize, Clone)]
struct DocumentData {
    id: String,          // Unique ID for each document
    title: String,
    content: String,
}

#[component]
pub fn App() -> impl IntoView {
    let (title_text, set_title_text) = create_signal(String::new());
    let (textbox_text, set_textbox_text) = create_signal(String::new());
    let (recent_documents, set_recent_documents) = create_signal(Vec::<DocumentData>::new());
    let (current_id, set_current_id) = create_signal(String::new()); // Signal for the unique ID

    // Generate a new unique ID when the app starts
    let generate_id = || uuid::Uuid::new_v4().to_string();
    create_effect(move |_| {
        if current_id.get().is_empty() {
            set_current_id.set(generate_id());
        }
    });

    // Auto-save function
    let auto_save = create_action(move |_| async move {
        let id = current_id.get_untracked();
        let title = title_text.get_untracked();
        let content = textbox_text.get_untracked();

        // Only save if there's content
        if !title.is_empty() || !content.is_empty() {
            let args = serde_wasm_bindgen::to_value(&json!({
                "id": id,
                "title": title,
                "content": content
            }))
            .unwrap();

            let _ = invoke("save_document", args).await;
        }
    });

    // Debounce auto-save (modify this to debounce less aggressively)
    create_effect(move |_| {
        let _title = title_text.get(); // Trigger dependency
        let _content = textbox_text.get(); // Trigger dependency

        set_timeout(move || {
            auto_save.dispatch(());
        }, Duration::from_millis(1000)); // Increased to 1 second to reduce file writes
    });

    let load_recent_documents = create_action(move |_| async move {
        let json_value = invoke("load_recent_files", JsValue::NULL).await;

        if let Some(json_str) = json_value.as_string() {
            if let Ok(docs) = serde_json::from_str::<Vec<DocumentData>>(&json_str) {
                if let Some(last_doc) = docs.last() {
                    set_current_id.set(last_doc.id.clone());
                    set_title_text.set(last_doc.title.clone());
                    set_textbox_text.set(last_doc.content.clone());
                }
                set_recent_documents.set(docs);
            }
        } else {
            print!("Failed to parse the response from `load_recent_files`.");
        }
    });

    // Load documents when component mounts
    create_effect(move |_| {
        load_recent_documents.dispatch(());
    });

    // Update title and textbox handlers
    let update_title = move |ev: ev::Event| {
        if let Some(input_event) = ev.dyn_into::<web_sys::InputEvent>().ok() {
            let v = event_target_value(&input_event);
            set_title_text.set(v);
        }
    };

    let update_textbox = move |ev: ev::Event| {
        if let Some(input_event) = ev.dyn_into::<web_sys::InputEvent>().ok() {
            let v = event_target_value(&input_event);
            set_textbox_text.set(v);
        }
    };
    // Delete document handler (Ctrl+X)
    let delete_document = move |ev: KeyboardEvent| {
        if ev.ctrl_key() && ev.key() == "x" {
            ev.prevent_default();
            let id = current_id.get_untracked();

            spawn_local(async move {
                let args = serde_wasm_bindgen::to_value(&json!({
                    "id": id
                }))
                .unwrap();

                let _ = invoke("delete_document", args).await;

                // Clear the current document
                set_current_id.set(String::new());
                set_title_text.set(String::new());
                set_textbox_text.set(String::new());
            });
        }
    };

    view! {
        <main class="container"
            on:keydown=delete_document
        >
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
                    placeholder="Start typing..."
                    value=textbox_text
                    on:input=update_textbox
                />
            </div>
        </main>
    }
}
