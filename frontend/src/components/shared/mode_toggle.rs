use yew::prelude::*;

#[derive(PartialEq, Clone, Copy)]
pub enum ToggleMode {
    Inspect,
    Define,
}

#[derive(Properties, PartialEq)]
pub struct ModeToggleProps {
    pub mode: ToggleMode,
    pub on_toggle: Callback<ToggleMode>,
}

#[function_component(ModeToggle)]
pub fn mode_toggle(props: &ModeToggleProps) -> Html {
    let ModeToggleProps { mode, on_toggle } = props;

    // Sliding indicator logic
    let indicator_transform = match mode {
        ToggleMode::Inspect => "translate-x-0",
        ToggleMode::Define => "translate-x-[calc(100%)]",
    };

    // Button style helpers
    let get_btn_classes = |active: bool| {
        classes!(
            "relative", "z-10", "px-4", "py-1.5", "text-xs", "font-mono", "font-semibold", "rounded-full",
            "transition-colors", "duration-200", "cursor-pointer",
            if active { 
                "text-primary-foreground" 
            } else { 
                "text-muted-foreground hover:text-foreground" 
            }
        )
    };

    let on_inspect = {
        let on_toggle = on_toggle.clone();
        Callback::from(move |_| on_toggle.emit(ToggleMode::Inspect))
    };

    let on_define = {
        let on_toggle = on_toggle.clone();
        Callback::from(move |_| on_toggle.emit(ToggleMode::Define))
    };

    html! {
        <div class="relative flex items-center bg-secondary/80 border border-border rounded-full p-0.5 backdrop-blur-sm">
            /* Sliding indicator */
            <div
                class={classes!(
                    "absolute", "top-0.5", "bottom-0.5", "w-[calc(50%-2px)]", "rounded-full",
                    "bg-primary", "shadow-[0_0_10px_hsl(var(--glow-primary))]",
                    "transition-transform", "duration-250", "ease-out",
                    indicator_transform
                )}
            />
            
            <button
                onclick={on_inspect}
                class={get_btn_classes(*mode == ToggleMode::Inspect)}
            >
                { "Inspect" }
            </button>

            <button
                onclick={on_define}
                class={get_btn_classes(*mode == ToggleMode::Define)}
            >
                { "Define" }
            </button>
        </div>
    }
}
