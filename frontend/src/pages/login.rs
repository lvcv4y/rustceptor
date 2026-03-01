use gloo::net::http::Request;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::{AuthContext, AuthState};
use crate::models::LoginRequest;

#[component(LoginPage)]
pub fn login_page() -> Html {
    let key = use_state(|| "".to_string());
    let auth = use_context::<AuthContext>().unwrap();

    let oninput = {
        let key: UseStateHandle<String> = key.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            key.set(input.value());
        })
    };

    
    let submit_callback = Callback::from(move |e: SubmitEvent| {
            e.prevent_default(); // Do not "send" the form: managed internally.
            
             // Rust is being that smartass kid
            let auth = auth.clone();
            let key = key.to_string();

            spawn_local(async move {
                let req = Request::post("/backapi/login")
                    .header("Content-Type", "application/json")
                    .body(serde_json::to_string(&LoginRequest { key : key }).unwrap_or("".to_string()))
                    .send().await;
                match req {
                    Ok(r) if r.status() == 200
                        => auth.set(AuthState::Authenticated),
                      _ => auth.set(AuthState::Unauthenticated) // TODO: add failed auth for error message
                }
            });
        });
    

    html! {
        <>
            <h1>{ "Login page" }</h1>
            <form onsubmit={submit_callback}>
                <input placeholder={ "Da master key" } name="key" oninput={oninput} />
                <button type={ "submit" }>{ "Login" }</button>
            </form>
        </>
    }
}