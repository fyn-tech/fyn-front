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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FlexAlign {
    Start,    
    Center,   
    End,      
    Stretch,  
    Baseline, 
}

impl std::fmt::Display for FlexAlign {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let class = match self {
            FlexAlign::Start => "items-start",
            FlexAlign::Center => "items-center",
            FlexAlign::End => "items-end", 
            FlexAlign::Stretch => "items-stretch",
            FlexAlign::Baseline => "items-baseline",
        };
        write!(f, "{}", class)
    }
}

// ------------------------------------------------------------------------------------------------
//  Spacing
// ------------------------------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Spacing{ // in px
    No, // 4 
    Xs, // 4 
    Sm, // 8 (default)
    Md, // 16
    Lg, // 24
}

impl std::fmt::Display for Spacing {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let class = match self {
            Spacing::No => "",
            Spacing::Xs => "1",
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

pub const ROUND_BORDER: &str = "rounded-lg";

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
            BorderColor::Surface => "border border-surface-200 dark:border-surface-800",
            BorderColor::Primary => "border border-primary-500",
            BorderColor::Success => "border border-success",
            BorderColor::Warning => "border border-warning", 
            BorderColor::Error => "border border-error",
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
    #[prop(default = true)] fill_space: bool,
    #[prop(default = FlexAlign::Stretch)] align: FlexAlign,
    #[prop(optional)] add_class: Option<String>,
    children: Children
) -> impl IntoView {
    
    let additional_class = add_class.unwrap_or_default();

    let class_str = format!("{} flex-{} gap-{} {} {}", 
        if fill_space {"flex"} else {"inline-flex"},
        if horizontal {"row"} else {"col"}, 
        space,
        align,
        additional_class
    );
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
    #[prop(default = true)] fill_space: bool,
    children: Children) -> impl IntoView{

    let col_str = if cols != 0 {format!("grid-cols-{}", cols)} else {"".to_string()};
    let row_str = if rows != 0 {format!("grid-rows-{}", rows)} else {"".to_string()};
    let class_str:String = format!("{} {} {} gap-{}", 
                                   if fill_space {"grid"} else {"inline-grid"},
                                   col_str, 
                                   row_str, 
                                   space).to_string();
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