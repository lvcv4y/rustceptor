use yew::prelude::*;
use web_sys::HtmlInputElement;
use lucide_yew::{Trash2, Save};

use crate::components::shared::app_button::{AppButton, Variant, Size};
use crate::components::shared::app_input::AppInput;
use crate::components::shared::app_textarea::AppTextArea;
use crate::components::shared::contenttype_selector::ContentTypeSelector;

use crate::models::RouteDefinition;

#[derive(Properties, PartialEq)]
pub struct DefinePanelProps {
    pub route: Option<RouteDefinition>,
    pub is_new: bool,
    pub on_save: Callback<RouteDefinition>,
    pub on_delete: Callback<String>,
}

#[function_component(DefinePanel)]
pub fn define_panel(props: &DefinePanelProps) -> Html {
    // Internal state for the form
    let form = use_state(|| RouteDefinition {
        route: "/".to_string(),
        description: "".to_string(),
        status_code: 200,
        content_type: "".to_string(),
        response_body: "".to_string(),
    });

    // Synchronize form state when the route prop changes
    {
        let form = form.clone();
        let route_prop = props.route.clone();
        use_effect_with(route_prop, move |route| {
            if let Some(r) = route {
                form.set(r.clone());
            } else {
                form.set(RouteDefinition {
                    route: "/".to_string(),
                    description: "".to_string(),
                    status_code: 200,
                    content_type: "".to_string(),
                    response_body: "".to_string(),
                });
            }
            || ()
        });
    }

    let on_save_click = {
        let on_save = props.on_save.clone();
        let form = form.clone();
        Callback::from(move |_| on_save.emit((*form).clone()))
    };

    // Render empty state
    if props.route.is_none() && !props.is_new {
        return html! {
            <div class="flex-1 flex items-center justify-center">
                <p class="text-sm font-mono text-muted-foreground">
                    { "Select a route or create a new one" }
                </p>
            </div>
        };
    }

    // TODO disable default route

    html! {
        <div class="flex-1 overflow-y-auto p-6">
            <div class="max-w-2xl mx-auto space-y-6">
                <div class="flex items-center justify-between">
                    <h2 class="text-lg font-mono font-semibold text-foreground">
                        { if props.is_new { "New Route" } else { "Edit Route" } }
                    </h2>
                    if !props.is_new {
                        <AppButton 
                            variant={Variant::Danger} 
                            size={Size::Small} 
                            onclick={
                                let on_delete = props.on_delete.clone();
                                let id = form.route.clone(); // Using route as ID placeholder
                                Callback::from(move |_| on_delete.emit(id.clone()))
                            }
                        >
                            <Trash2 size=14 />
                            { "Delete" }
                        </AppButton>
                    }
                </div>

                <div class="flex gap-3 items-end">
                    <div class="flex-1">
                        <AppInput
                            label="Path"
                            value={form.route.clone()}
                            placeholder="/capture/path"
                            oninput={
                                let form = form.clone();
                                Callback::from(move |e: InputEvent| {
                                    let input: HtmlInputElement = e.target_unchecked_into();
                                    let mut new_form = (*form).clone();
                                    new_form.route = input.value();
                                    form.set(new_form);
                                })
                            }
                        />
                    </div>
                    <div class="w-24">
                        <AppInput
                            label="Status"
                            value={form.status_code.to_string()}
                            oninput={
                                let form = form.clone();
                                Callback::from(move |e: InputEvent| {
                                    let input: HtmlInputElement = e.target_unchecked_into();
                                    match input.value().parse::<u16>() {
                                        Ok(i) => {
                                            let mut new_form = (*form).clone();
                                            new_form.status_code = i;
                                            form.set(new_form);
                                        },
                                        _ => ()
                                    }
                                })
                            }
                            placeholder="200"
                        />
                    </div>
                </div>

                <ContentTypeSelector
                    value={form.content_type.clone()}
                    on_change={
                        let form = form.clone();
                        Callback::from(move |new_value: String| {
                            let mut new_form = (*form).clone();
                            new_form.content_type = new_value;
                            form.set(new_form);
                        })
                    }
                />

                <AppInput
                    label="Description"
                    value={form.description.clone()}
                    placeholder="Short description"
                    oninput={
                        let form = form.clone();
                        Callback::from(move |e: InputEvent| {
                            let input: HtmlInputElement = e.target_unchecked_into();
                            let mut new_form = (*form).clone();
                            new_form.description = input.value();
                            form.set(new_form);
                        })
                    }
                />

                <AppTextArea
                    label="Response Body"
                    value={form.response_body.clone()}
                    placeholder=r#"{"message": "success"}"#
                    class="min-h-[200px]"
                    oninput={
                        let form = form.clone();
                        Callback::from(move |e: InputEvent| {
                            let input: HtmlInputElement = e.target_unchecked_into();
                            let mut new_form = (*form).clone();
                            new_form.response_body = input.value();
                            form.set(new_form);
                        })
                    }
                />

                <AppButton 
                    variant={Variant::Primary} 
                    size={Size::Large} 
                    onclick={on_save_click} 
                    class="w-full"
                >
                    <Save size=16 />
                    { if props.is_new { "Create Route" } else { "Save Changes" } }
                </AppButton>
            </div>
        </div>
    }
}