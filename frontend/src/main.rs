use gloo::net::http::Request;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

mod pages;
mod models;
mod components;

use crate::components::utils::toast::{ToastContext, ToastState, ToastOverlay};
use crate::pages::home::HomePage;
use crate::pages::login::LoginPage;

/*
 * Auth structure
 */

#[derive(Clone, PartialEq)]
pub enum AuthState {
    Authenticated,
    Unauthenticated,
    Loading
}

pub type AuthContext = UseStateHandle<AuthState>;

/*
 * Routes
 */

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/home")]
    Home,

    #[at("/login")]
    Login,

    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(route: Route, auth: &AuthState) -> Html {
    match (&auth, route) {
        (_, Route::NotFound) => html! { <p>{ "Not found :(" }</p> },
        (AuthState::Unauthenticated, _) => html! { <LoginPage /> },
        (AuthState::Loading, _) => html! { <p> { "Loading..." } </p> },
        
        (_, Route::Login) => html! { <LoginPage /> },
        (AuthState::Authenticated, Route::Home) => html! { <HomePage /> },
    }
}

/*
 * App
 */

#[component]
fn App() -> Html {
    let auth: AuthContext = use_state(|| AuthState::Loading);
    let toast: ToastContext = use_reducer(|| ToastState { displaying: Vec::new()});


    // Send once a request to the API to determine the auth status.
    {
        let auth = auth.clone();
        use_effect_with((), move |_| {
            spawn_local(async move {
                let req = Request::get("/backapi/me")
                    .credentials(web_sys::RequestCredentials::Include)
                    .send()
                    .await;

                match req {
                    Ok(r) if r.status() == 200
                        => auth.set(AuthState::Authenticated),
                      _ => auth.set(AuthState::Unauthenticated),
                      // TODO add 500 case
                }
            });
            || ()
        });
    }

    html! {
        <ContextProvider<ToastContext> context={toast.clone()}>
            <ToastOverlay />
            <ContextProvider<AuthContext> context={auth.clone()}>
                <BrowserRouter>
                    <Switch<Route> render={ move |route| switch(route, &*auth) } />
                </BrowserRouter>
            </ContextProvider<AuthContext>>
        </ContextProvider<ToastContext>>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
