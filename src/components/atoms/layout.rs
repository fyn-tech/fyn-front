use std::fmt;
use leptos::prelude::*;

// ------------------------------------------------------------------------------------------------
//  Alignment
// ------------------------------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Align {
    Left,    
    Center,  
    Right,   
    Justify, 
}

impl std::fmt::Display for Align {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let class = match self {
            Align::Left => "text-left",
            Align::Center => "text-center", 
            Align::Right => "text-right",
            Align::Justify => "text-justify",
        };
        return write!(f, "{}", class);
    }
}

// ------------------------------------------------------------------------------------------------
//  Spacing
// ------------------------------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Spacing{
    Sm, // 8 (default)
    Md, // 16
    Lg, // 24
}

impl std::fmt::Display for Spacing {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let class = match self {
            Spacing::Sm => "2",
            Spacing::Md => "3",
            Spacing::Lg => "4"
        };
        return write!(f, "{}", class);
    }
}

// ------------------------------------------------------------------------------------------------
//  Boarders
// ------------------------------------------------------------------------------------------------

pub const ROUND_BORDER: &str = "rounded-lg border";

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BorderColor {
    Surface,  
    Primary,  
    Success,  
    Warning,  
    Error,    
}

impl std::fmt::Display for BorderColor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let class = match self {
            BorderColor::Surface => "border-surface-200 dark:border-surface-700",
            BorderColor::Primary => "border-primary-500",
            BorderColor::Success => "border-success",
            BorderColor::Warning => "border-warning", 
            BorderColor::Error => "border-error",
        };
        return write!(f, "{}", class);
    }
}

pub fn standard_border(color: Option<BorderColor>) -> String {
    let color = color.unwrap_or(BorderColor::Surface);
    return format!("{} {}", ROUND_BORDER, color);
}

// ------------------------------------------------------------------------------------------------
//  Components
// ------------------------------------------------------------------------------------------------

#[component]
pub fn Stack(
    #[prop(default = Spacing::Sm)] space: Spacing,
    #[prop(default = false)] horizontal: bool,
    children: Children
) -> impl IntoView {
    
    let class_str:String = format!("flex flex-{} gap-{}", 
                                   if horizontal {"row"} else {"col"}, space).to_string();
    return view!{
        <div class={class_str}>
            {children()}
        </div>
    };
}

#[component]
pub fn Grid(
    #[prop(default = Spacing::Sm)] space: Spacing,
    #[prop(default = 0)] cols: u8,
    #[prop(default = 0)] rows: u8,
    children: Children) -> impl IntoView{

    let col_str = if cols != 0 {format!("grid-cols-{}", cols)} else {"".to_string()};
    let row_str = if rows != 0 {format!("grid-rows-{}", rows)} else {"".to_string()};
    let class_str:String = format!("grid {} {} gap-{}", col_str, row_str, space).to_string();
    return view!{
        <div class={class_str}>
            {children()}
        </div>
    };
}

#[component]
pub fn BorderedDiv(
    #[prop(default = BorderColor::Surface)] border: BorderColor,
    #[prop(optional)] class: Option<String>,
    children: Children,
) -> impl IntoView {
    let additional_class = class.unwrap_or_default();
    
    view! {
        <div class=format!("overflow-hidden {} {}", standard_border(Some(border)), additional_class)>
            {children()}
        </div>
    }
}