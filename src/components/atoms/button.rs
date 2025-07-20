use leptos::prelude::*;
use crate::components::atoms::typography::{NORMAL_CLASS};
use crate::components::atoms::layout::*;

// ------------------------------------------------------------------------------------------------
//  Variant
// ------------------------------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
pub enum Variant {
    Primary,      
    Secondary,    
    Tertiary,     
    Success,      
    Warning,      
}

impl std::fmt::Display for Variant{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result{
        let class = match self{
            Variant::Primary => 
                "bg-primary-500 dark:bg-primary-950 hover:bg-primary-300 dark:hover:bg-primary-700",
            Variant::Secondary => 
                "bg-primary-300 dark:bg-primary-700 hover:bg-primary-50 dark:hover:bg-primary-500",
            Variant::Tertiary => 
                "bg-accent-300 dark:bg-accent-500 hover:bg-accent-50 dark:hover:bg-accent-300",
            Variant::Success => "bg-semantic-success",
            Variant::Warning => "bg-semantic-warning",
        };
        return write!(f, "{}", class);
    }

}      

// ------------------------------------------------------------------------------------------------
//  Components
// ------------------------------------------------------------------------------------------------

#[component]
pub fn Button(
    #[prop(default = Variant::Primary)] variant: Variant,
    #[prop(default = "Click me".to_string())] text: String,
    #[prop(optional)] on_click: Option<Box<dyn Fn() + 'static>>,
) -> impl IntoView {
    let padding = format!("px-{} py-{}", Spacing::Md, Spacing::Sm);
    
    let button_classes = format!("{} {} {} {}", variant, ROUND_BORDER, NORMAL_CLASS, padding);

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