use leptos::prelude::*;
use crate::components::atoms::layout::{Align};

pub const H1_CLASS: &str = "font-primary font-bold text-2xl text-content-primary dark:text-content-primary-dark";
pub const H2_CLASS: &str = "font-primary font-semibold text-xl text-content-primary dark:text-content-primary-dark";
pub const H3_CLASS: &str = "font-primary font-medium text-lg text-content-primary dark:text-content-primary-dark";
pub const H4_CLASS: &str = "font-primary font-medium text-base text-content-primary dark:text-content-primary-dark";
pub const NORMAL_CLASS: &str = "font-primary font-sm text-base text-content-primary dark:text-content-primary-dark";

#[component]
pub fn H1(
    #[prop(default = Align::Left)] align: Align,
    children: Children
) -> impl IntoView {
    return view! {
        <h1 class=format!("{} {}", H1_CLASS, align)>
            {children()}
        </h1>
    };
}

#[component]
pub fn H2(
    #[prop(default = Align::Left)] align: Align,
    children: Children
) -> impl IntoView {
    return view! {
        <h2 class=format!("{} {}", H2_CLASS, align)>
            {children()}
        </h2>
    };
}

#[component]
pub fn H3(
    #[prop(default = Align::Left)] align: Align,
    children: Children
) -> impl IntoView {
    return view! {
        <h3 class=format!("{} {}", H3_CLASS, align)>
            {children()}
        </h3>
    };
}

#[component]
pub fn H4(
    #[prop(default = Align::Left)] align: Align,
    children: Children
) -> impl IntoView {
    return view! {
        <h4 class=format!("{} {}", H4_CLASS, align)>
            {children()}
        </h4>
    };
}

#[component]
pub fn P(
    #[prop(default = Align::Justify)] align: Align,
    children: Children
) -> impl IntoView {
    return view! {
        <p class=format!("{} {}", NORMAL_CLASS, align)>
            {children()}
        </p>
    };
}