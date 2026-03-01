use yew::prelude::*;

use crate::components::ClassVariant;

#[derive(PartialEq, Default)]
#[allow(dead_code)]
pub enum Variant {
    #[default]
    Primary,
    Secondary,
    Ghost,
    Danger,
    Icon
}

#[derive(PartialEq, Default)]
pub enum Size {
    Small,
    #[default]
    Medium,
    Large,
    Icon
}

impl ClassVariant for Variant {
    fn to_class_str(self: &Self) -> &'static str {
        match *self {
            Self::Primary => "bg-primary text-primary-foreground hover:brightness-110 shadow-[0_0_12px_hsl(var(--glow-primary))]",
            Self::Secondary => "bg-secondary text-secondary-foreground hover:bg-secondary/80 border border-border",
            Self::Ghost => "bg-transparent text-foreground hover:bg-secondary/60",
            Self::Danger => "bg-destructive text-destructive-foreground hover:brightness-110",
            Self::Icon => "bg-transparent text-muted-foreground hover:text-foreground hover:bg-secondary/60",
        }
    }
}

impl ClassVariant for Size {
    fn to_class_str(self: &Self) -> &'static str {
        match *self {
            Self::Small => "px-3 py-1.5 text-xs",
            Self::Medium => "px-4 py-2 text-sm",
            Self::Large => "px-6 py-2.5 text-sm",
            Self::Icon => "p-2"
        }
    }
}

#[derive(PartialEq, Properties)]
pub struct AppButtonProps {
    #[prop_or_default]
    pub children: Html,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or(false)]
    pub disabled: bool,

    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,

    #[prop_or_default]
    pub size: Size,

    #[prop_or_default]
    pub variant: Variant,
}

#[component]
pub fn AppButton(props: &AppButtonProps) -> Html {
    let AppButtonProps {
        children,
        class,
        disabled,
        onclick,
        size,
        variant,
    } = props;

    let class = classes![
        String::from(r#"
            inline-flex items-center justify-center gap-2 rounded-md font-mono font-medium
            transition-all duration-150 cursor-pointer
            focus:outline-none focus:ring-2 focus:ring-ring/40
            disabled:opacity-40 disabled:pointer-events-none
        "#),
        variant.classes(),
        size.classes(),
        class.clone(),
    ];

    html! {
        <button class={class} disabled={*disabled} {onclick}>
            {children.clone()}
        </button>
    }
}
