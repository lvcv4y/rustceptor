use yew::prelude::*;
use lucide_yew::X;

use crate::components::shared::app_button::{AppButton, Variant, Size};

#[derive(Properties, PartialEq)]
pub struct SettingsPanelProps {
    pub open: bool,
    pub on_close: Callback<MouseEvent>,
}

#[function_component(SettingsPanel)]
pub fn settings_panel(props: &SettingsPanelProps) -> Html {
    // If not open, render nothing immediately
    if !props.open {
        return html! {};
    }

    html! {
        <>
            /* Backdrop */
            <div
                class="fixed inset-0 z-50 transition-all bg-background/40 backdrop-blur-sm"
                onclick={props.on_close.clone()}
            />
            
            /* Drawer */
            <div
                class="fixed right-0 top-0 bottom-0 z-50 w-80 bg-card border-l border-border shadow-2xl flex flex-col"
            >
                <div class="flex items-center justify-between p-4 border-b border-border">
                    <h2 class="text-sm font-mono font-semibold text-foreground">{ "Settings" }</h2>
                    <AppButton 
                        variant={Variant::Icon} 
                        size={Size::Icon} 
                        onclick={props.on_close.clone()}
                    >
                        <X size=16 />
                    </AppButton>
                </div>
                
                <div class="flex-1 p-4 space-y-4">
                    <p class="text-xs font-mono text-muted-foreground">
                        { "Settings panel - Coming soon (not really but)..." }
                    </p>
                </div>
            </div>
        </>
    }
}