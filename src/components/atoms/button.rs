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

impl Variant{
    fn base_colour(&self) -> &str{
        return match self{
            Variant::Primary => "bg-primary-500 dark:bg-primary-950",
            Variant::Secondary => "bg-primary-300 dark:bg-primary-700",
            Variant::Tertiary => "bg-accent-300 dark:bg-accent-500",
            Variant::Success => "bg-semantic-success",
            Variant::Warning => "bg-semantic-warning",
        };
    }

    fn hover_colour(&self) -> &str{
        return match self{
            Variant::Primary => "hover:bg-primary-300 dark:hover:bg-primary-700",
            Variant::Secondary => "hover:bg-primary-50 dark:hover:bg-primary-500",
            Variant::Tertiary => "hover:bg-accent-50 dark:hover:bg-accent-300",
            Variant::Success => "",
            Variant::Warning => "",
        };
    }
}      

// ------------------------------------------------------------------------------------------------
//  State
// ------------------------------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
pub enum State {
    Default,
    Loading,     // Shows spinner
    Disabled,    // Non-interactive
    Active,      // Currently selected/pressed
}

// ------------------------------------------------------------------------------------------------
//  Components
// ------------------------------------------------------------------------------------------------

#[component]
pub fn Button(
    #[prop(default = Variant::Primary)] variant: Variant,
    #[prop(default = State::Default)] state: State,
    #[prop(default = "Click me".to_string())] text: String,
    #[prop(optional)] on_click: Option<Box<dyn Fn() + 'static>>,
) -> impl IntoView {
    let padding = format!("px-{} py-{}", Spacing::Md, Spacing::Sm);
    
    let (state_modifiers, hover) = match state {
        State::Default => ("", variant.hover_colour()),
        State::Active => ("ring-2 ring-primary-300", variant.hover_colour()),
        State::Disabled => ("opacity-50 cursor-not-allowed", ""),
        State::Loading => ("opacity-75", ""),
    };

    let button_classes = format!("{} {} {} {} {} {}", variant.base_colour(), hover, state_modifiers, ROUND_BORDER, NORMAL_CLASS, padding);

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