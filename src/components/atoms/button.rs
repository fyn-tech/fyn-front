use leptos::prelude::*;
use crate::components::atoms::typography::{FONT_CLR, FONT_STR, Size as TextSize};
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
    pub fn base_colour(&self) -> &str{
        return match self{
            Variant::Primary => "bg-primary-500 dark:bg-primary-950",
            Variant::Secondary => "bg-primary-300 dark:bg-primary-700",
            Variant::Tertiary => "bg-accent-300 dark:bg-accent-500",
            Variant::Success => "bg-semantic-success",
            Variant::Warning => "bg-semantic-warning",
        };
    }

    pub fn hover_colour(&self) -> &str{
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
//  Type
// ------------------------------------------------------------------------------------------------

// TODO TYPES: will add as we go along.
#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Standard,    // Default clickable button
    Toggle,       // On/off state button
    Radio,        // Single selection from group
    Checkbox,     // Multiple selection
}

// ------------------------------------------------------------------------------------------------
//  Size
// ------------------------------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
pub enum Size {
    Xs, 
    Sm, 
    Md, 
    Lg, 
    Xl, 
}

fn padding(size: &Size) -> String {
    return match size {
        Size::Xs => format!("px-{} py-{}", Spacing::Xs, Spacing::Xs),
        Size::Sm => format!("px-{} py-{}", Spacing::Sm, Spacing::Xs),
        Size::Md => format!("px-{} py-{}", Spacing::Md, Spacing::Sm),
        Size::Lg => format!("px-{} py-{}", Spacing::Lg, Spacing::Md),
        Size::Xl => format!("px-{} py-{}", Spacing::Lg, Spacing::Lg),
    };
}


// ------------------------------------------------------------------------------------------------
//  State
// ------------------------------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
pub enum State {
    Default,
    Loading,
    Disabled,
    Active, 
}

// ------------------------------------------------------------------------------------------------
//  Components
// ------------------------------------------------------------------------------------------------

#[component]
pub fn Button(
    #[prop(default = Variant::Primary)] variant: Variant,
    #[prop(default = State::Default)] state: State,
    #[prop(default = Size::Md)] size: Size,
    #[prop(default = "Click me".to_string())] text: String,
    #[prop(optional)] on_click: Option<Box<dyn Fn() + 'static>>,
) -> impl IntoView {
    
    let padding = padding(&size);
    
    let (state_modifiers, hover) = match state {
        State::Default => ("", variant.hover_colour()),
        State::Active => ("ring-2 ring-primary-950 dark:ring-primary-300", variant.hover_colour()),
        State::Disabled => ("opacity-50 cursor-not-allowed", ""),
        State::Loading => ("opacity-75", ""),
    };

    let text_format = format!("{} {} {}", 
        FONT_STR,
        match size {
            Size::Xs => TextSize::Xs,
            Size::Sm => TextSize::Sm,
            Size::Md => TextSize::Base,
            Size::Lg => TextSize::Lg,
            Size::Xl => TextSize::Xl,    
        }, 
        FONT_CLR);

    let button_classes = format!("{} {} {} {} {} {}", variant.base_colour(), hover, state_modifiers,
                                 ROUND_BORDER, padding, text_format);

    return view! {
        <button 
            id=format!("btn-{:?}", size)
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

#[component]
pub fn GroupButton(
    #[prop(default = State::Default)] state: State,
    #[prop(default = Size::Md)] size: Size,
    #[prop(default = "Click me".to_string())] text: String,
    #[prop(optional)] on_click: Option<Box<dyn Fn() + 'static>>,
) -> impl IntoView {
    
    let padding = padding(&size);
    
    let (state_modifiers, hover) = match state {
        State::Default => ("", Variant::Primary.hover_colour()),
        State::Active => ("ring-2 ring-primary-300", Variant::Primary.hover_colour()),
        State::Disabled => ("opacity-50 cursor-not-allowed", ""),
        State::Loading => ("opacity-75", ""),
    };

    let text_format = format!("{} {} {}", 
        FONT_STR,
        match size {
            Size::Xs => TextSize::Xs,
            Size::Sm => TextSize::Sm,
            Size::Md => TextSize::Base,
            Size::Lg => TextSize::Lg,
            Size::Xl => TextSize::Xl,    
        }, 
        FONT_CLR);

    let button_classes = format!("{} {} {} {}", hover, state_modifiers,
                                 padding, text_format);

    return view! {
        <button 
            id=format!("btn-{:?}", size)
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