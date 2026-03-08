use yew::prelude::*;
use lucide_yew::Menu;

use crate::components::shared::app_button::{AppButton, Variant, Size};
use crate::components::shared::mode_toggle::{ModeToggle, ToggleMode};


#[derive(Properties, PartialEq)]
pub struct NavbarProps {
    pub mode: ToggleMode,
    pub on_mode_change: Callback<ToggleMode>,
    pub on_settings_click: Callback<MouseEvent>,
}

#[function_component(Navbar)]
pub fn navbar(props: &NavbarProps) -> Html {
    let NavbarProps { 
        mode, 
        on_mode_change, 
        on_settings_click 
    } = props;

    html! {
        <nav
            class="fixed top-0 left-0 right-0 z-50 flex items-center justify-between \
                   px-6 py-3 bg-card/60 backdrop-blur-xl border-b border-border/50"
        >
            <div class="flex items-center gap-3">
                <span class="text-sm font-mono font-bold text-primary tracking-tight">
                    { "⚡ RUSTCEPTOR" }
                </span>
            </div>

            <ModeToggle 
                mode={*mode} 
                on_toggle={on_mode_change.clone()} 
            />

            <AppButton 
                variant={Variant::Icon} 
                size={Size::Icon} 
                onclick={on_settings_click.clone()}
            >
                <Menu size=20 />
            </AppButton>
        </nav>
    }
}