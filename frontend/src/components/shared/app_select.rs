use yew::prelude::*;

#[allow(dead_code)]
#[derive(PartialEq, Clone)]
pub struct SelectOption {
    pub value: String,
    pub label: String,
}

#[allow(dead_code)]
#[derive(Properties, PartialEq)]
pub struct AppSelectProps {
    pub options: Vec<SelectOption>,
    
    #[prop_or_default]
    pub label: Option<String>,
    
    #[prop_or_default]
    pub id: Option<String>,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub value: String,

    #[prop_or_default]
    pub onchange: Callback<Event>,

    #[prop_or_default]
    pub disabled: bool,
}

#[function_component(AppSelect)]
pub fn app_select(props: &AppSelectProps) -> Html {
    let AppSelectProps {
        options,
        label,
        id,
        class,
        value,
        onchange,
        disabled,
    } = props;

    let input_id = id.clone().unwrap_or_else(|| {
        label
            .as_ref()
            .map(|l| l.to_lowercase().replace(' ', "-"))
            .unwrap_or_default()
    });

    let select_classes = classes![
        "w-full", "rounded-md", "bg-secondary", "border", "border-border", "px-3", "py-2",
        "text-sm", "font-mono", "text-foreground",
        "focus:outline-none", "focus:ring-2", "focus:ring-ring/40", "focus:border-primary/50",
        "transition-all", "duration-150", "cursor-pointer", "appearance-none",
        "bg-no-repeat", "bg-[right_0.75rem_center]",
        // Handling the SVG background via raw string to avoid escaping hell
        String::from(r#"bg-[url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='12' height='12' viewBox='0 0 24 24' fill='none' stroke='%2364748b' stroke-width='2'%3E%3Cpath d='m6 9 6 6 6-6'/%3E%3C/svg%3E")]"#),
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
            <select
                id={input_id}
                class={select_classes}
                value={value.clone()}
                onchange={onchange.clone()}
                disabled={*disabled}
            >
                {
                    options.iter().map(|opt| {
                        html! {
                            <option key={opt.value.clone()} value={opt.value.clone()}>
                                { &opt.label }
                            </option>
                        }
                    }).collect::<Html>()
                }
            </select>
        </div>
    }
}