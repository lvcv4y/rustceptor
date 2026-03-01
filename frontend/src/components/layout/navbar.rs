use yew::prelude::*;
// Assuming these are in your project structure
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
                // Lucide-style Menu Icon
                <svg 
                    xmlns="http://www.w3.org/2000/svg" 
                    width="20" 
                    height="20" 
                    viewBox="0 0 24 24" 
                    fill="none" 
                    stroke="currentColor" 
                    stroke-width="2" 
                    stroke-linecap="round" 
                    stroke-linejoin="round" 
                    class="w-5 h-5"
                >
                    <line x1="4" x2="20" y1="12" y2="12" />
                    <line x1="4" x2="20" y1="6" y2="6" />
                    <line x1="4" x2="20" y1="18" y2="18" />
                </svg>
            </AppButton>
        </nav>
    }
}