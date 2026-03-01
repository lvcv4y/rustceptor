use yew::prelude::*;

use crate::components::shared::app_button::{AppButton, Variant, Size};
use crate::models::RouteDefinition;


#[derive(Properties, PartialEq)]
pub struct RouteSidebarProps {
    pub routes: Vec<RouteDefinition>,

    #[prop_or(None)]
    pub selected_route_id: Option<String>,
    
    pub on_select_route: Callback<String>,
    pub on_add_route: Callback<MouseEvent>, // Matches AppButton's expected Callback<MouseEvent>
}

#[function_component(RouteSidebar)]
pub fn route_sidebar(props: &RouteSidebarProps) -> Html {
    let routes = &props.routes;

    html! {
        <aside class="w-64 shrink-0 h-full border-r border-border bg-card/40 flex flex-col">
            <div class="p-3 border-b border-border">
                <span class="text-[10px] font-mono text-muted-foreground uppercase tracking-widest">
                    { "Routes" }
                </span>
            </div>

            <div class="flex-1 overflow-y-auto">
                {
                    if routes.is_empty() {
                        html! {
                            <div class="p-4 text-center">
                                <p class="text-xs font-mono text-muted-foreground">{ "No routes defined" }</p>
                            </div>
                        }
                    } else {
                        routes.iter().map(|route| {
                            let route_id = route.route.clone();
                            let on_click_cb = props.on_select_route.clone();
                            
                            let is_selected = props.selected_route_id.as_ref() == Some(&route.route);
                            
                            let dynamic_classes = if is_selected {
                                "bg-primary/10 border-l-2 border-l-primary"
                            } else {
                                "hover:bg-secondary/60 border-l-2 border-l-transparent"
                            };

                            html! {
                                <button
                                    key={route.route.clone()}
                                    onclick={Callback::from(move |_| on_click_cb.emit(route_id.clone()))}
                                    class={classes!(
                                        "w-full", "text-left", "px-3", "py-2.5", "flex", "items-center", "gap-2.5",
                                        "border-b", "border-border/50", "transition-colors", "duration-100", "cursor-pointer",
                                        dynamic_classes
                                    )}
                                >
                                    <span class="text-xs font-mono text-foreground truncate">{ if route.route == "/backapi/*" { "*" } else { &route.route }}</span>
                                </button>
                            }
                        }).collect::<Html>()
                    }
                }
            </div>

            <div class="p-3 border-t border-border">
                <AppButton 
                    variant={Variant::Primary} 
                    size={Size::Small} 
                    class="w-full" 
                    onclick={props.on_add_route.clone()}
                >
                    // "plus" icon 
                    <svg 
                        xmlns="http://www.w3.org/2000/svg" 
                        width="14" 
                        height="14" 
                        viewBox="0 0 24 24" 
                        fill="none" 
                        stroke="currentColor" 
                        stroke-width="2" 
                        stroke-linecap="round" 
                        stroke-linejoin="round" 
                        class="w-3.5 h-3.5"
                    >
                        <path d="M5 12h14"/><path d="M12 5v14"/>
                    </svg>
                    { "Add Route" }
                </AppButton>
            </div>
        </aside>
    }
}