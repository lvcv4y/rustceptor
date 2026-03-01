use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct AppInputProps {
    #[prop_or_default]
    pub label: Option<String>,
    
    #[prop_or_default]
    pub id: Option<String>,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub value: String,

    #[prop_or_default]
    pub placeholder: String,

    #[prop_or_default]
    pub oninput: Callback<InputEvent>,

    #[prop_or_default]
    pub input_type: String, // Maps to "type" attribute
}

#[function_component(AppInput)]
pub fn app_input(props: &AppInputProps) -> Html {
    let AppInputProps {
        label,
        id,
        class,
        value,
        placeholder,
        oninput,
        input_type,
    } = props;

    // Logic for generating the ID if not provided, mimicking the React version
    let input_id = id.clone().unwrap_or_else(|| {
        label
            .as_ref()
            .map(|l| l.to_lowercase().replace(' ', "-"))
            .unwrap_or_default()
    });

    let input_classes = classes![
        "w-full", "rounded-md", "bg-secondary", "border", "border-border", "px-3", "py-2",
        "text-sm", "font-mono", "text-foreground", "placeholder:text-muted-foreground",
        "focus:outline-none", "focus:ring-2", "focus:ring-ring/40", "focus:border-primary/50",
        "transition-all", "duration-150",
        class.clone()
    ];

    html! {
        <div class="flex flex-col gap-1.5">
            if let Some(label_text) = label {
                <label 
                    for={input_id.clone()} 
                    class="text-xs font-mono text-muted-foreground uppercase tracking-wider"
                >
                    { label_text }
                </label>
            }
            <input
                id={input_id}
                type={input_type.clone()}
                class={input_classes}
                value={value.clone()}
                placeholder={placeholder.clone()}
                oninput={oninput.clone()}
            />
        </div>
    }
}
