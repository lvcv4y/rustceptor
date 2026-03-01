use yew::prelude::*;
use uuid::Uuid;
use gloo_timers::callback::Timeout;
use lucide_yew::{CircleAlert, TriangleAlert, CircleCheck, Info, Bug, X};
use std::rc::Rc;

use crate::components::ClassVariant;

// Data structs: holds the actual data.
#[derive(PartialEq, Default, Clone)]
#[allow(dead_code)]
pub enum ToastVariant {
    #[default]
    Info,
    Success,
    Warning,
    Error,
    Debug
}

impl ClassVariant for ToastVariant {
    fn to_class_str(self: &Self) -> &'static str {
        match *self {
            Self::Info    => "notif-info",
            Self::Warning => "notif-warning",
            Self::Success => "notif-success",
            Self::Error   => "notif-error",
            Self::Debug   => "notif-debug",
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct Toast {
    pub id: String,
    pub variant: ToastVariant,
    pub title: String,
    pub message: Option<String>,
    pub duration: u32,
}

impl Toast {
    pub fn new(variant: ToastVariant, title: &str, message: Option<&str>) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            variant,
            title: title.to_string(),
            message: message.map(|m| m.to_string()),
            duration: 4000,
        }
    }
}

// Toast context: used by components to add toasts
#[derive(PartialEq, Clone)]
pub struct ToastState {
    pub displaying: Vec<Toast>
}

pub type ToastContext = UseReducerHandle<ToastState>;

/* 
 * Use a Reducer architecture to prevent race condition between durations of
 * different toasts
 */
pub enum ToastAction {
    Add(Toast),
    Remove(String),
}

impl Reducible for ToastState {
    type Action = ToastAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut displaying = self.displaying.clone();
        match action {
            ToastAction::Add(toast) => {
                displaying.push(toast);
            }
            ToastAction::Remove(id) => {
                displaying.retain(|t| t.id != id);
            }
        }
        Self { displaying }.into()
    }
}

// Toast component: represents what will be displayed.
#[derive(Properties, PartialEq)]
pub struct ToastComponentProps {
    pub toast: Toast,
    pub on_close: Callback<String>,
}
#[function_component(ToastComponent)]
pub fn toast_component(props: &ToastComponentProps) -> Html {
    let exiting = use_state(|| false);
    let toast = &props.toast;
    
    // Triggers exit animation and toast removal
    let handle_dismiss = {
        let exiting = exiting.clone();
        let on_close = props.on_close.clone();
        let id = toast.id.clone();
        
        Callback::from(move |_| {
            if !*exiting {
                exiting.set(true);
                let on_close = on_close.clone();
                let id = id.clone();
                Timeout::new(450, move || {
                    on_close.emit(id);
                }).forget();
            }
        })
    };

    // Auto-dismiss once the timeout expires.
    {
        let handle_dismiss = handle_dismiss.clone();
        let duration = toast.duration;
        let id = toast.id.clone();
        use_effect_with(id, move |_| {
            let timeout: Timeout = Timeout::new(duration, move || {
                handle_dismiss.emit(MouseEvent::new("click").unwrap());
            });
            move || drop(timeout)  // Delete the timeout if the toast get manually removed by the user.
        });
    }

    let icon = match toast.variant {
        ToastVariant::Error   => html! { <CircleAlert   class="notif-icon shrink-0 mt-0.5" size=18 stroke_width=2 /> },
        ToastVariant::Warning => html! { <TriangleAlert class="notif-icon shrink-0 mt-0.5" size=18 stroke_width=2 /> },
        ToastVariant::Success => html! { <CircleCheck   class="notif-icon shrink-0 mt-0.5" size=18 stroke_width=2 /> },
        ToastVariant::Info    => html! { <Info          class="notif-icon shrink-0 mt-0.5" size=18 stroke_width=2 /> },
        ToastVariant::Debug   => html! { <Bug           class="notif-icon shrink-0 mt-0.5" size=18 stroke_width=2 /> },
    };

    let animation_class = if *exiting { "notif-exit" } else { "notif-enter" };

    html! {
        <div class={classes!(
            animation_class,
            toast.variant.to_class_str(),
            "border", "rounded-xl", "px-4", "py-3", "shadow-lg", "shadow-black/5",
            "flex", "items-start", "gap-3", "w-80", "relative", "overflow-hidden", "mt-3"
        )}>
            {icon}
            
            <div class="flex-1 min-w-0">
                <p class="notif-title font-semibold text-sm leading-tight">{ &toast.title }</p>
                {if let Some(msg) = &toast.message {
                    html! { <p class="notif-message text-xs mt-1 opacity-75 leading-snug">{ msg }</p> }
                } else {
                    html! {}
                }}
            </div>

            <button
                onclick={handle_dismiss}
                class="notif-close opacity-50 hover:opacity-100 transition-opacity shrink-0 mt-0.5 cursor-pointer"
            >
                <X size=14 />
            </button>

            <div class="absolute bottom-0 left-0 right-0 h-[3px] bg-black/5">
                <div
                    class="h-full notif-progress-bar notif-progress"
                    style={format!("animation-duration: {}ms", toast.duration)}
                />
            </div>
        </div>
    }
}

/*
 * Overlay container for toasts. Basically a container that lays in front of everything.
 * Where it sits defines where the toast will appear.
 */
#[derive(PartialEq, Properties)]
pub struct ToastOverlayProps {}

#[component]
pub fn ToastOverlay(props: &ToastOverlayProps) -> Html {
    let ToastOverlayProps {} = props;

    let ctx: ToastContext = use_context().unwrap();

    let remove_toast = {
        let ctx = ctx.clone();
        Callback::from(move |id: String| {
            ctx.dispatch(ToastAction::Remove(id));
        })
    };
    
    html! {
        <div class={classes!["fixed bottom-6 right-6 z-50 flex flex-col-reverse items-end".to_string()]} >
            {
                for ctx.displaying.iter().map(|toast| {
                    let remove_toast = remove_toast.clone();
                    html! {
                        <ToastComponent 
                            key={toast.id.clone()}
                            toast={toast.clone()}
                            on_close={remove_toast} 
                        />
                    }
                })
            }
        </div>
    }
}

// Pub function to emit toast (standardize toast emission)
pub fn emit_toast(ctx: ToastContext, toast: Toast) {
    // TODO use custom hook? or Macro?
    ctx.dispatch(ToastAction::Add(toast));
}