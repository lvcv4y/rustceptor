use gloo::net::http::Request;
use wasm_bindgen_futures::spawn_local;
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast; // Required for .unchecked_ref()

use web_sys::{EventSource, EventSourceInit, MessageEvent};
use yew::prelude::*;

use std::rc::Rc;

use crate::components::layout::navbar::*;
use crate::components::panels::define_panel::*;
use crate::components::panels::inspect_panel::*;
use crate::components::layout::route_sidebar::RouteSidebar;
use crate::components::panels::settings_panel::*;
use crate::components::shared::mode_toggle::ToggleMode;
use crate::models::DeleteRouteRequest;
use crate::models::RouteListRequest;
use crate::models::{CapturedRequestFetch, RouteDefinition, UpdateRouteRequest, CapturedRequest};

use crate::components::utils::toast::{ToastContext, Toast, ToastVariant, emit_toast};

fn default_route() -> RouteDefinition {
    RouteDefinition {
        route: "/backapi/*".to_string(), // Any route ; use of impossible value, so we can filter it out. TODO cleaner version.
        description: "Default route (immutable for now)".to_string(),
        status_code: 200,
        content_type: "".to_string(),
        response_body: "".to_string(),
    }
}

#[derive(Clone)]
enum RequestUpdateAction {
    Add(CapturedRequest),
    Set(Vec<CapturedRequest>),
}

#[derive(PartialEq, Clone)]
struct RequestList {
    pub inner: Vec<CapturedRequest>,
}

impl Reducible for RequestList {
    type Action = RequestUpdateAction;
    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            RequestUpdateAction::Add(req) => {
                let mut v = (self.inner).clone();
                v.push(req);
                RequestList {inner: v}.into()
            },
            RequestUpdateAction::Set(reqvec) => {
                RequestList{ inner: reqvec.clone() }.into()
            }
        }
    }
}

#[function_component(HomePage)]
pub fn main_page() -> Html {

    // --- State ---

    let toast_ctx: ToastContext = use_context().unwrap();
    let mode: UseStateHandle<ToggleMode> = use_state(|| ToggleMode::Define);
    
    // Route list using the updated RouteDefinition name
    let routes = use_state(|| vec![
        default_route()
    ]);

    {
        let routes = routes.clone();
        use_effect_with((), move |_| {
            spawn_local(async move {
                let req = Request::get("/backapi/routes")
                    .credentials(web_sys::RequestCredentials::Include)
                    .send().await;

                if let Ok(r) = req {
                    let json = r.json::<RouteListRequest>().await;

                    if let Ok(decoded) = json {
                        routes.set(vec![default_route()].into_iter().chain(decoded.routes).collect());
                    }
                }
            });
        });
    }

    let requests = use_reducer(|| RequestList{ inner: vec![]});
    // Fetch already captured requests
    {
        let requests = requests.clone();
        use_effect_with((), move |_| {  // Only once per run
            spawn_local(async move {
                let req = Request::get("/backapi/requests")
                    .credentials(web_sys::RequestCredentials::Include)
                    .send().await;

                if let Ok(r) = req {
                    let json = r.json::<CapturedRequestFetch>().await;

                    if let Ok(decoded) = json {
                        requests.dispatch(RequestUpdateAction::Set(decoded.requests));
                    }
                }
            });
            || ()
        });
    }

    // Listen for incoming request event
    {
        let requests = requests.clone();
        
        let init = EventSourceInit::new();
        init.set_with_credentials(true);

        use_effect_with((), move |_| {
            let source = EventSource::new_with_event_source_init_dict(
                "/backapi/listen/requests", 
                &init
            ).unwrap();
            let closure = Closure::wrap(Box::new(move |m: MessageEvent| {
                if let Some(json) = m.data().as_string() {
                    if let Ok(req) = serde_json::from_str::<CapturedRequest>(&json) {
                        requests.dispatch(RequestUpdateAction::Add(req));
                    }
                }
            }) as Box<dyn FnMut(MessageEvent)>);

            source.add_event_listener_with_callback("message",closure.as_ref().unchecked_ref()).unwrap();
            let _closure = closure.forget();

            move || {
                source.close();
                // Leak?
                // drop(closure);
            }
        });
    }

    let selected_route_id = use_state(|| None);
    let is_new_route = use_state(|| false);
    let settings_open = use_state(|| false);

    // --- Selectors ---
    
    let selected_route = {
        let routes = (*routes).clone();
        let current_id = (*selected_route_id).clone();
        routes.into_iter().find(|r| Some(r.route.clone()) == current_id)
    };

    // --- Handlers ---

    let handle_add_route = {
        let is_new = is_new_route.clone();
        let selected_id = selected_route_id.clone();
        Callback::from(move |_| {
            is_new.set(true);
            selected_id.set(None);
        })
    };

    let handle_save_route = {
        let routes = routes.clone();
        let selected_route_id = selected_route_id.clone();
        let is_new_route = is_new_route.clone();

        let toast_ctx = toast_ctx.clone();

        Callback::from(move |updated_route: RouteDefinition| {
            let routes = routes.clone();
            let selected_route_id = selected_route_id.clone();
            let is_new_route = is_new_route.clone();
            let mut current_routes = (*routes).clone();

            let toast_ctx = toast_ctx.clone();
            
            spawn_local(async move {
                let req = Request::post("/backapi/add")
                    .credentials(web_sys::RequestCredentials::Include)
                    .header("Content-Type", "application/json")
                    .body(serde_json::to_string(&UpdateRouteRequest{ 
                        target_route: (*selected_route_id).clone(),
                        definition: updated_route.clone(),
                    }).unwrap_or("".to_string()))
                    .send().await;

                match req {
                    Ok(r) if r.status() == 200 || r.status() == 201 => {
                        // Update current_route (either add or change existing RouteDefinition)

                        if let Some(current_route) = (*selected_route_id).clone() {
                        // Match based on the 'route' string field
                            if let Some(pos) = current_routes.iter().position(|r| r.route == current_route) {
                                current_routes[pos] = updated_route.clone();
                            } else {  // Isn't supposed to happen
                                current_routes.push(updated_route.clone());
                            }
                        } else {
                            // current_route_id is None: this is a new route
                            current_routes.push(updated_route.clone());
                        }

                        selected_route_id.set(Some(updated_route.route));
                        routes.set(current_routes);
                        is_new_route.set(false);

                        emit_toast(toast_ctx.clone(), Toast::new(
                            ToastVariant::Success,
                            "Success",
                            Some({
                                if r.status() == 200 {
                                    "Changes saved."
                                } else {  // 201: Created
                                    "The route has been successfully created!"
                                }
                            })
                        ));
                    }
                    _ => emit_toast(toast_ctx, Toast::new(
                            ToastVariant::Error,
                            "Oops!", 
                            Some("An error occured when trying to add/update the route. Maybe check the values (like the path) you entered.")
                        )),
                }
            });  
        })
    };

    let handle_delete_route = {
        let routes = routes.clone();
        let selected_route_id = selected_route_id.clone();

        let toast_ctx = toast_ctx.clone();
        
        Callback::from(move |id: String| {
            let routes = routes.clone();
            let selected_route_id = selected_route_id.clone();

            let toast_ctx = toast_ctx.clone();
        
            spawn_local(async move {
                let req = Request::post("/backapi/delete")
                    .header("Content-Type", "application/json")
                    .credentials(web_sys::RequestCredentials::Include)
                    .body(serde_json::to_string(&DeleteRouteRequest {
                        route: id.clone()
                    }).unwrap_or("".to_string()))
                    .send().await;

                match req {
                    Ok(r) if r.status() == 200 => {
                        let mut current_routes = (*routes).clone();
                        current_routes.retain(|r| r.route != id);
                        routes.set(current_routes);
                        selected_route_id.set(None);

                        emit_toast(toast_ctx.clone(), Toast::new(
                            ToastVariant::Success,
                            "Goodbye",
                            Some("The route has been successfully deleted!")
                        ));
                    },
                    _ => emit_toast(toast_ctx, Toast::new(
                            ToastVariant::Error,
                            "Oops!", 
                            Some("The route deletion failed. What did you try to do huh?")
                        ))
                }
            });
        })
    };

    let handle_select_route = {
        let selected_id = selected_route_id.clone();
        let is_new = is_new_route.clone();
        Callback::from(move |id: String| {
            selected_id.set(Some(id));
            is_new.set(false);
        })
    };

    // --- Render ---

    html! {
        <div class="h-screen flex flex-col bg-background overflow-hidden">
            <Navbar 
                mode={(*mode).clone()} 
                on_mode_change={
                    let mode = mode.clone();
                    Callback::from(move |m| mode.set(m))
                }
                on_settings_click={
                    let settings_open = settings_open.clone();
                    Callback::from(move |_| settings_open.set(true))
                }
            />

            <div class="flex flex-1 pt-[52px] overflow-hidden">
                <RouteSidebar 
                    routes={(*routes).clone()}
                    selected_route_id={(*selected_route_id).clone()}
                    on_select_route={handle_select_route}
                    on_add_route={handle_add_route}
                />

                {
                    match *mode {
                        ToggleMode::Define => html! {
                            <DefinePanel 
                                route={if *is_new_route { None } else { selected_route }}
                                is_new={*is_new_route}
                                on_save={handle_save_route}
                                on_delete={handle_delete_route}
                            />
                        },
                        ToggleMode::Inspect => html! {
                            <InspectPanel 
                                requests={requests.inner.clone()} 
                                selected_route_id={(*selected_route_id).clone()} 
                            />
                        }
                    }
                }
            </div>

            <SettingsPanel 
                open={*settings_open} 
                on_close={
                    let settings_open = settings_open.clone();
                    Callback::from(move |_| settings_open.set(false))
                } 
            />
        </div>
    }
}