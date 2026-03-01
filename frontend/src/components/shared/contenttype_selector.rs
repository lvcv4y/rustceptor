use yew::prelude::*;
use web_sys::HtmlInputElement;

// Reuse the SelectOption struct or define locally
#[derive(PartialEq, Clone)]
pub struct ContentTypeOption {
    pub value: &'static str,
    pub label: &'static str,
}

pub const COMMON_TYPES: [ContentTypeOption; 5] = [
    ContentTypeOption { value: "application/json", label: "application/json" },
    ContentTypeOption { value: "application/x-www-form-urlencoded", label: "application/x-www-form-urlencoded" },
    ContentTypeOption { value: "text/plain", label: "text/plain" },
    ContentTypeOption { value: "text/html", label: "text/html" },
    ContentTypeOption { value: "multipart/form-data", label: "multipart/form-data" },
];

#[derive(Properties, PartialEq)]
pub struct ContentTypeSelectorProps {
    pub value: String,
    pub on_change: Callback<String>,
}

#[function_component(ContentTypeSelector)]
pub fn content_type_selector(props: &ContentTypeSelectorProps) -> Html {
    let ContentTypeSelectorProps { value, on_change } = props;
    
    // Check if current value is a preset
    let is_custom_init = !COMMON_TYPES.iter().any(|t| t.value == value);
    let custom_mode = use_state(|| is_custom_init);
    let input_ref = use_node_ref();

    // Effect: Focus input when entering custom mode
    {
        let input_ref = input_ref.clone();
        let custom_mode_val = *custom_mode;
        use_effect_with(custom_mode_val, move |mode| {
            if *mode {
                if let Some(input) = input_ref.cast::<HtmlInputElement>() {
                    let _ = input.focus();
                }
            }
            || ()
        });
    }

    // Handlers
    let on_input_change = {
        let on_change = on_change.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            on_change.emit(input.value());
        })
    };

    let on_select_change = {
        let on_change = on_change.clone();
        Callback::from(move |e: Event| {
            let select: web_sys::HtmlInputElement = e.target_unchecked_into();
            on_change.emit(select.value());
        })
    };

    let toggle_custom = {
        let custom_mode = custom_mode.clone();
        Callback::from(move |_| custom_mode.set(true))
    };

    let toggle_presets = {
        let custom_mode = custom_mode.clone();
        let on_change = on_change.clone();
        Callback::from(move |_| {
            custom_mode.set(false);
            on_change.emit("application/json".to_string()); // TODO ...probably cause of him. Deleting doesn't fix though.
        })
    };

    // TODO bug: when using "custom", the content-type value showed is not the actual saved one.
    //      it gets override by "application/json"...
    html! {
        <div class="flex flex-col gap-1.5">
            <label class="text-xs font-mono text-muted-foreground uppercase tracking-wider">
                { "Content-Type" }
            </label>
            
            if *custom_mode {
                <div class="flex gap-2">
                    <input
                        ref={input_ref}
                        value={value.clone()}
                        oninput={on_input_change}
                        placeholder="application/custom"
                        class="flex-1 rounded-md bg-secondary border border-border px-3 py-2
                               text-sm font-mono text-foreground placeholder:text-muted-foreground
                               focus:outline-none focus:ring-2 focus:ring-ring/40 focus:border-primary/50
                               transition-all duration-150"
                    />
                    <button
                        type="button"
                        onclick={toggle_presets}
                        class="text-xs font-mono text-primary hover:text-primary/80 px-2 shrink-0 transition-colors"
                    >
                        { "Presets" }
                    </button>
                </div>
            } else {
                <div class="flex gap-2">
                    <select
                        value={value.clone()}
                        onchange={on_select_change}
                        class={classes!(
                            "flex-1", "rounded-md", "bg-secondary", "border", "border-border", "px-3", "py-2",
                            "text-sm", "font-mono", "text-foreground",
                            "focus:outline-none", "focus:ring-2", "focus:ring-ring/40", "focus:border-primary/50",
                            "transition-all", "duration-150", "cursor-pointer", "appearance-none",
                            "bg-no-repeat", "bg-[right_0.75rem_center]",
                            String::from(r#"bg-[url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='12' height='12' viewBox='0 0 24 24' fill='none' stroke='%2364748b' stroke-width='2'%3E%3Cpath d='m6 9 6 6 6-6'/%3E%3C/svg%3E")]"#)
                        )}
                    >
                        {
                            COMMON_TYPES.iter().map(|t| {
                                html! {
                                    <option key={t.value} value={t.value} selected={t.value == value}>
                                        { t.label }
                                    </option>
                                }
                            }).collect::<Html>()
                        }
                    </select>
                    <button
                        type="button"
                        onclick={toggle_custom}
                        class="text-xs font-mono text-primary hover:text-primary/80 px-2 shrink-0 transition-colors"
                    >
                        { "Custom" }
                    </button>
                </div>
            }
        </div>
    }
}