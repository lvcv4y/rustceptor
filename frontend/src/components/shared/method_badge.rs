use yew::prelude::*;
use crate::models::HttpMethod;
use crate::components::ClassVariant; // Adjust path as needed

impl ClassVariant for HttpMethod {
    fn to_class_str(&self) -> &'static str {
        match self {
            HttpMethod::Post => "text-method-post border-method-post/30 bg-method-post/10",
            HttpMethod::Put => "text-method-put border-method-put/30 bg-method-put/10",
            HttpMethod::Delete => "text-method-delete border-method-delete/30 bg-method-delete/10",
            HttpMethod::Patch => "text-method-patch border-method-patch/30 bg-method-patch/10",
            // GET and all other variants use the GET style
            _ => "text-method-get border-method-get/30 bg-method-get/10",
        }
    }
}

// Since HttpMethod likely doesn't implement Display by default in your models,
// we can handle the text rendering in a helper or via a match.
impl HttpMethod {
    pub fn to_str(&self) -> &'static str {
        match self {
            HttpMethod::Get => "GET",
            HttpMethod::Head => "HEAD",
            HttpMethod::Options => "OPTIONS",
            HttpMethod::Trace => "TRACE",
            HttpMethod::Put => "PUT",
            HttpMethod::Delete => "DELETE",
            HttpMethod::Post => "POST",
            HttpMethod::Patch => "PATCH",
            HttpMethod::Connect => "CONNECT",
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct MethodBadgeProps {
    pub method: HttpMethod,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(MethodBadge)]
pub fn method_badge(props: &MethodBadgeProps) -> Html {
    let MethodBadgeProps { method, class } = props;

    let badge_classes = classes![
        "inline-flex", "items-center", "justify-center", "rounded", "px-2", "py-0.5",
        "text-[10px]", "font-mono", "font-bold", "uppercase", "tracking-wider",
        "border",
        method.classes(),
        class.clone()
    ];

    html! {
        <span class={badge_classes}>
            { method.to_str() }
        </span>
    }
}