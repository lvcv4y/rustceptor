use yew::prelude::*;
use std::collections::HashMap;
use lucide_yew::{ChevronRight, ClockAlert};

use crate::components::shared::method_badge::MethodBadge; 
use crate::models::{CapturedRequest};

#[derive(Properties, PartialEq)]
pub struct InspectPanelProps {
    pub requests: Vec<CapturedRequest>,
    pub selected_route_id: Option<String>,
}

#[function_component(InspectPanel)]
pub fn inspect_panel(props: &InspectPanelProps) -> Html {
    let selected_request_id = use_state(|| Option::<String>::None);

    // Filtering logic
    let filtered: Vec<&CapturedRequest> =
    (if props.selected_route_id == Some("/backapi/*".to_string()) {
        // That's the all route, everything matches.
        // TODO refactor to make it a "default" route. instead of a "all" one?
        props.requests.iter().collect::<Vec<&CapturedRequest>>()
    } else {
        props.requests.iter()
            .filter(|r| {
                if let Some(target_route) = &props.selected_route_id {
                    // Matching target_route against the route field.
                    r.route.as_ref() == Some(target_route)
                } else {
                    false
                }
            })
        .collect()
    }).into_iter().rev().collect();   // Reverse the list, so it matches arrival order

    // Find selected request
    let selected_request = selected_request_id.as_ref().and_then(|id| {
        filtered.iter().find(|r| r.uuid == *id)
    });

    if filtered.is_empty() {
        return html! {
            <div class="flex-1 flex items-center justify-center">
                <div class="text-center space-y-2">
                    <div class="w-10 h-10 rounded-full bg-secondary border border-border flex items-center justify-center mx-auto">
                        <ClockAlert size=20 class="text-muted-foreground" />
                    </div>
                    <p class="text-sm font-mono text-muted-foreground">
                        { if props.selected_route_id.is_some() { "No captured requests on this route" } else { "No captured requests yet" } }
                    </p>
                </div>
            </div>
        };
    }
    
    let mut headers = if let Some(req) = selected_request {
        req.headers.clone()
    } else {
        HashMap::new()
    };

    let cookies = {
        let cookie_opt = headers.remove("cookie");
        if let Some(cstr) = cookie_opt {
            cstr.split(";").filter_map(|kv| {
                let (k, v) = kv.trim().split_once("=")?;
                Some((k.trim().to_string(), v.trim().to_string()))
            }).collect()
        } else {
            HashMap::new()
        }
    };

    html! {
        <div class="flex-1 flex overflow-hidden">
            /* Request list */
            <div class="w-72 shrink-0 border-r border-border overflow-y-auto pt-1">
                {
                    for filtered.clone().into_iter().map(|req| {
                        let req_id = req.uuid.clone();
                        let current_id = req_id.clone();
                        let is_active = *selected_request_id == Some(req_id.clone());
                        let onclick = {
                            let selected_request_id = selected_request_id.clone();
                            Callback::from(move |_| selected_request_id.set(Some(req_id.clone())))
                        };

                        html! {
                            <button
                                key={current_id}
                                {onclick}
                                class={classes!(
                                    "w-full", "text-left", "px-3", "py-3", "flex", "items-center", "gap-2",
                                    "border-b", "border-border/50", "transition-colors", "cursor-pointer",
                                    if is_active { "bg-primary/10" } else { "hover:bg-secondary/60" }
                                )}
                            >
                                <MethodBadge method={req.method.clone()} />
                                <div class="flex-1 min-w-0">
                                    <p class="text-xs font-mono text-foreground truncate">
                                        { req.route.as_deref().unwrap_or("/") }
                                    </p>
                                    <p class="text-[10px] font-mono text-muted-foreground mt-0.5">
                                        { req.timestamp.clone() }
                                    </p>
                                </div>
                                <ChevronRight size=14 />
                            </button>
                        }
                    })
                }
            </div>

            /* Request detail */
            <div class="flex-1 overflow-y-auto">
                if let Some(req) = selected_request {
                    <div class="p-6 space-y-5">
                        <div class="flex items-center gap-3">
                            <MethodBadge method={req.method.clone()} />
                            <span class="text-sm font-mono font-semibold text-foreground">
                                { req.route.as_deref().unwrap_or("/") }
                            </span>
                        </div>

                        if let Some(ip_str) = &req.client_ip {
                            <DetailSection title="Client IP">
                                <p class="text-xs font-mono text-foreground">{ ip_str.clone() }</p>
                            </DetailSection>
                        }

                        <DetailSection title="Headers">
                            <KeyValueTable data={headers} />
                        </DetailSection>

                        <DetailSection title="Query Parameters">
                            <KeyValueTable data={req.query_parameters.clone()} />
                        </DetailSection>

                        <DetailSection title="Cookies">
                            <KeyValueTable data={cookies} />
                        </DetailSection>

                        <DetailSection title="Request Body">
                            // TODO: check if empty
                            <pre class="text-xs font-mono text-foreground bg-secondary rounded-md p-3 overflow-x-auto border border-border">
                                // TODO: Body is base64-encoded in struct; decode it here
                                { req.body.as_deref().unwrap_or("(empty)") }
                            </pre>
                        </DetailSection>
                    </div>
                } else {
                    <div class="flex-1 h-full flex items-center justify-center">
                        <p class="text-sm font-mono text-muted-foreground">{ "Select a request to inspect" }</p>
                    </div>
                }
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct DetailSectionProps {
    title: String,
    children: Html,
}

#[function_component(DetailSection)]
fn detail_section(props: &DetailSectionProps) -> Html {
    html! {
        <div class="space-y-2">
            <h3 class="text-[10px] font-mono text-muted-foreground uppercase tracking-widest">
                { &props.title }
            </h3>
            { props.children.clone() }
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct KeyValueTableProps {
    data: HashMap<String, String>,
}

#[function_component(KeyValueTable)]
fn key_value_table(props: &KeyValueTableProps) -> Html {
    if props.data.len() == 0 {
        return html! { <p class="text-xs font-mono text-muted-foreground italic">{ "None" }</p> };
    }

    html! {
        <div class="rounded-md border border-border overflow-hidden">
            {
                for props.data.iter().enumerate().map(|(i, (key, value))| {
                    html! {
                        <div key={key.clone()} class={classes!(
                            "flex", "text-xs", "font-mono",
                            if i > 0 { "border-t border-border/50" } else { "" }
                        )}>
                            <span class="px-3 py-1.5 bg-secondary/60 text-primary w-48 shrink-0 truncate">
                                { key }
                            </span>
                            <span class="px-3 py-1.5 text-foreground flex-1 truncate">
                                // TODO maybe copy on click / double-click? Might be convenient
                                { value }
                            </span>
                        </div>
                    }
                })
            }
        </div>
    }
}