use const_format::formatcp;
use leptos::prelude::*;

use crate::common::size::*;
use crate::components::atoms::layout::Align;

// ------------------------------------------------------------------------------------------------
//  TextSizing
// ------------------------------------------------------------------------------------------------

pub const fn text_size(size: Size) -> &'static str {
    return match size {
        Size::None => "",
        Size::Xs => "text-xs",
        Size::Sm => "text-sm",
        Size::Md => "text-base",
        Size::Lg => "text-lg",
        Size::Xl => "text-xl",
        Size::Xl2 => "text-2xl",
    };
}

// ------------------------------------------------------------------------------------------------
//  Text Formatting
// ------------------------------------------------------------------------------------------------

pub const FONT_LGT_CLR: &str = "text-content-primary";
pub const FONT_DRK_CLR: &str = "text-content-primary-dark";
pub const FONT_CLR: &str = formatcp!("{} dark:{}", FONT_LGT_CLR, FONT_DRK_CLR);
pub const LINK_CLR: &str =
    "text-accent-500 hover:text-accent-300 dark:text-accent-300 dark:hover:text-accent-50";

pub const FONT_STR: &str = "font-primary";
pub const H1_CLASS: &str = formatcp!("{} {} {}", FONT_STR, "font-bold", text_size(Size::Xl2));
pub const H2_CLASS: &str = formatcp!("{} {} {}", FONT_STR, "font-semibold", text_size(Size::Xl));
pub const H3_CLASS: &str = formatcp!("{} {} {}", FONT_STR, "font-medium", text_size(Size::Lg));
pub const H4_CLASS: &str = formatcp!("{} {} {}", FONT_STR, "font-medium", text_size(Size::Md));
pub const NORMAL_CLASS: &str = formatcp!("{} {} {}", FONT_STR, "font-sm", text_size(Size::Md));

// ------------------------------------------------------------------------------------------------
//  Components
// ------------------------------------------------------------------------------------------------

#[component]
pub fn H1(
    #[prop(default = Align::Left)] align: Align,
    #[prop(default = FONT_CLR.to_string())] color: String,
    children: Children,
) -> impl IntoView {
    return view! {
        <h1 class=format!("{} {} {}", H1_CLASS, color, align)>
            {children()}
        </h1>
    };
}

#[component]
pub fn H2(
    #[prop(default = Align::Left)] align: Align,
    #[prop(default = FONT_CLR.to_string())] color: String,
    children: Children,
) -> impl IntoView {
    return view! {
        <h2 class=format!("{} {} {}", H2_CLASS, color, align)>
            {children()}
        </h2>
    };
}

#[component]
pub fn H3(
    #[prop(default = Align::Left)] align: Align,
    #[prop(default = FONT_CLR.to_string())] color: String,
    children: Children,
) -> impl IntoView {
    return view! {
        <h3 class=format!("{} {} {}", H3_CLASS, color, align)>
            {children()}
        </h3>
    };
}

#[component]
pub fn H4(
    #[prop(default = Align::Left)] align: Align,
    #[prop(default = FONT_CLR.to_string())] color: String,
    children: Children,
) -> impl IntoView {
    return view! {
        <h4 class=format!("{} {} {}", H4_CLASS, color, align)>
            {children()}
        </h4>
    };
}

#[component]
pub fn P(
    #[prop(default = Align::Justify)] align: Align,
    #[prop(default = FONT_CLR.to_string())] color: String,
    children: Children,
) -> impl IntoView {
    return view! {
        <p class=format!("{} {} {}", NORMAL_CLASS, color, align)>
            {children()}
        </p>
    };
}

#[component]
pub fn A(
    href: String,
    text_class: String,
    #[prop(default = Align::Justify)] align: Align,
    #[prop(default = LINK_CLR.to_string())] color: String,
    children: Children,
) -> impl IntoView {
    return view! {
        <a href={href} class=format!("{} {} {}", text_class, color, align)>
            {children()}
        </a>
    };
}
