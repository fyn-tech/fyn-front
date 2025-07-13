use leptos::prelude::*;
use crate::components::atoms::typography::{NORMAL_CLASS};
use crate::components::atoms::layout::*;

#[component]
pub fn Button(
    #[prop(default = "primary".to_string())] variant: String,
    #[prop(default = "Click me".to_string())] text: String,
    #[prop(optional)] on_click: Option<Box<dyn Fn() + 'static>>,
) -> impl IntoView {
    let padding = format!("px-{} py-{}", Spacing::Md, Spacing::Sm);
    let hover = "hover:bg-primary-50 dark:hover:bg-primary-950";
    
    let button_classes = match variant.as_str() {
        "primary" => format!("bg-primary-500 {} {} {} {}", hover, ROUND_BORDER, NORMAL_CLASS, padding),
        "secondary" => format!("bg-surface-200 dark:bg-surface-800 hover:bg-surface-300 dark:hover:bg-surface-700
         {} {} {}", standard_border(None), NORMAL_CLASS, padding),
        _ => format!("bg-primary-500 {} {} {} {}", hover, ROUND_BORDER, NORMAL_CLASS, padding),
    };

    return view! {
        <button 
            class={button_classes}
            on:click=move |_| {
                if let Some(ref action) = on_click {
                    action();
                }
            }
        >
            {text}
        </button>
    };
}