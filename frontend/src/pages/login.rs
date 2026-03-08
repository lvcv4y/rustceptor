use gloo::net::http::Request;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::Redirect;

use crate::{AuthContext, AuthState, Route};
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

    
    let submit_callback = {
        let auth = auth.clone();
        
        Callback::from(move |e: SubmitEvent| {
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
        })
    };

    if *auth == AuthState::Authenticated {
        html! { <Redirect<Route> to={Route::Home} /> }
    } else {
        html! {
            <div class="min-h-screen flex items-center justify-center p-4">
                // Background glow using primary color
                <div class="absolute w-72 h-72 rounded-full bg-primary/10 blur-3xl" />
            
                <div class="relative z-10 w-full max-w-sm p-8 bg-card/70 backdrop-blur-md border rounded-[var(--radius)] shadow-xl">
                    <div class="flex flex-col space-y-2 text-center mb-8">
                        <h1 class="text-2xl font-semibold tracking-tight">
                            { "Welcome back" }
                        </h1>
                        <p class="text-sm text-muted-foreground">
                            { "Wanna do some funny things today?" }
                        </p>
                    </div>

                    <form onsubmit={submit_callback} class="space-y-6">
                        <div class="space-y-2">
                            <input 
                                name="key"
                                type="password"
                                placeholder="Da master key"
                                oninput={oninput}
                                required=true
                                class=r#"flex h-10 w-full rounded-[var(--radius)] border border-input bg-background px-3 py-2
                                         text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium
                                         placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring
                                         focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50"#
                            />
                        </div>

                        <button 
                            type="submit" 
                            class=r#"inline-flex items-center justify-center w-full h-10 px-4 py-2 bg-primary text-primary-foreground
                                    hover:opacity-90 rounded-[var(--radius)] font-medium transition-colors
                                    focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring shadow-sm"#
                        >
                            { "Login" }
                        </button>
                    </form>
                </div>
            </div>
        }
    }
}