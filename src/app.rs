use ev::KeyboardEvent;
// use leptos::leptos_dom::ev::SubmitEvent;
use leptos::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use std::time::Duration;
use serde_json::json;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize, Clone)]
struct DocumentData {
    title: String,
    content: String,
}

#[component]
pub fn App() -> impl IntoView {
    let (title_text, set_title_text) = create_signal(String::new());
    let (textbox_text, set_textbox_text) = create_signal(String::new());
    let (recent_documents, set_recent_documents) = create_signal(Vec::<DocumentData>::new());

    // Auto-save function
    let auto_save = create_action(move |_| async move {
        let title = title_text.get_untracked();
        let content = textbox_text.get_untracked();

        // Only save if there's content
        if !title.is_empty() || !content.is_empty() {
            let args = serde_wasm_bindgen::to_value(&json!({
                "title": title,
                "content": content
            })).unwrap();

            let _ = invoke("save_document", args).await;
        }
    });

    // Debounce auto-save
    create_effect(move |_| {
        let title = title_text.get();
        let content = textbox_text.get();
        
        set_timeout(move || {
            auto_save.dispatch(());
        }, Duration::from_millis(500));
    });

    let load_recent_documents = create_action(move |_| async move {
        let json_value = invoke("load_recent_files", JsValue::NULL).await;
    
        if let Some(json_str) = json_value.as_string() {
            if let Ok(docs) = serde_json::from_str::<Vec<DocumentData>>(&json_str) {
                // Load the most recently modified/last opened document
                if let Some(last_doc) = docs.last() {
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

    let update_title = move |ev: web_sys::InputEvent| {
        let v = event_target_value(&ev);
        set_title_text.set(v);
    };
    
    let update_textbox = move |ev: web_sys::InputEvent| {
        let v = event_target_value(&ev);
        set_textbox_text.set(v);
    };

    // Delete document handler (Ctrl+X)
    let delete_document = move |ev: KeyboardEvent| {
        if ev.ctrl_key() && ev.key() == "x" {
            ev.prevent_default();
            let title = title_text.get_untracked();
            let safe_filename = format!("{}.json", title);
            
            spawn_local(async move {
                let args = serde_wasm_bindgen::to_value(&json!({
                    "filename": safe_filename
                })).unwrap();

                let _ = invoke("delete_document", args).await;
                
                // Clear the current document
                set_title_text.set(String::new());
                set_textbox_text.set(String::new());
            });
        }
    };

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
                    placeholder="start typing..."
                    value=textbox_text
                    on:input=update_textbox
                />
            </div>
        </main>
    }
}
