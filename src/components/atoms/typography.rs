use const_format::formatcp;
use leptos::prelude::*;

use crate::components::atoms::layout::Align;

// ------------------------------------------------------------------------------------------------
//  TextSizing
// ------------------------------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Size {
    None,
    Xs,
    Sm,
    Base,
    Lg,
    Xl,
    Xl2,
}

impl Size {
    pub const fn as_str(&self) -> &'static str {
        return match self {
            Size::None => "",
            Size::Xs => "text-xs",
            Size::Sm => "text-sm",
            Size::Base => "text-base",
            Size::Lg => "text-lg",
            Size::Xl => "text-xl",
            Size::Xl2 => "text-2xl",
        };
    }
}

impl std::fmt::Display for Size {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        return write!(f, "{}", self.as_str());
    }
}

// ------------------------------------------------------------------------------------------------
//  Text Formatting
// ------------------------------------------------------------------------------------------------

pub const FONT_CLR: &str = "text-content-primary dark:text-content-primary-dark";
pub const LINK_CLR: &str =
    "text-accent-500 hover:text-accent-300 dark:text-accent-300 dark:hover:text-accent-50";

pub const FONT_STR: &str = "font-primary";
pub const H1_CLASS: &str = formatcp!("{} {} {}", FONT_STR, "font-bold", Size::Xl2.as_str());
pub const H2_CLASS: &str = formatcp!("{} {} {}", FONT_STR, "font-semibold", Size::Xl.as_str());
pub const H3_CLASS: &str = formatcp!("{} {} {}", FONT_STR, "font-medium", Size::Lg.as_str());
pub const H4_CLASS: &str = formatcp!("{} {} {}", FONT_STR, "font-medium", Size::Base.as_str());
pub const NORMAL_CLASS: &str = formatcp!("{} {} {}", FONT_STR, "font-sm", Size::Base.as_str());

// ------------------------------------------------------------------------------------------------
//  Components
// ------------------------------------------------------------------------------------------------

#[component]
pub fn H1(#[prop(default = Align::Left)] align: Align, children: Children) -> impl IntoView {
    return view! {
        <h1 class=format!("{} {} {}", H1_CLASS, FONT_CLR, align)>
            {children()}
        </h1>
    };
}

#[component]
pub fn H2(#[prop(default = Align::Left)] align: Align, children: Children) -> impl IntoView {
    return view! {
        <h2 class=format!("{} {} {}", H2_CLASS, FONT_CLR, align)>
            {children()}
        </h2>
    };
}

#[component]
pub fn H3(#[prop(default = Align::Left)] align: Align, children: Children) -> impl IntoView {
    return view! {
        <h3 class=format!("{} {} {}", H3_CLASS, FONT_CLR, align)>
            {children()}
        </h3>
    };
}

#[component]
pub fn H4(#[prop(default = Align::Left)] align: Align, children: Children) -> impl IntoView {
    return view! {
        <h4 class=format!("{} {} {}", H4_CLASS, FONT_CLR, align)>
            {children()}
        </h4>
    };
}

#[component]
pub fn P(#[prop(default = Align::Justify)] align: Align, children: Children) -> impl IntoView {
    return view! {
        <p class=format!("{} {} {}", NORMAL_CLASS, FONT_CLR, align)>
            {children()}
        </p>
    };
}

#[component]
pub fn A(
    href: String,
    text_class: String,
    #[prop(default = Align::Justify)] align: Align,
    children: Children,
) -> impl IntoView {
    return view! {
        <a href={href} class=format!("{} {} {}", text_class, LINK_CLR, align)>
            {children()}
        </a>
    };
}
