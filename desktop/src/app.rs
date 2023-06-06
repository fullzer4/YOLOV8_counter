use leptos::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[component]
pub fn App(cx: Scope) -> impl IntoView {

    view! { cx,
        <main class="container">
            <div>
            </div>
        </main>
    }
}
