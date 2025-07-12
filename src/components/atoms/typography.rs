use leptos::prelude::*;

pub const H1_CLASS: &str = "font-primary font-bold text-2xl text-content-primary dark:text-content-primary-dark";
pub const H2_CLASS: &str = "font-primary font-semibold text-xl text-content-primary dark:text-content-primary-dark";
pub const H3_CLASS: &str = "font-primary font-medium text-lg text-content-primary dark:text-content-primary-dark";
pub const H4_CLASS: &str = "font-primary font-medium text-base text-content-primary dark:text-content-primary-dark";
pub const NORMAL_CLASS: &str = "font-primary font-medium text-base text-content-primary dark:text-content-primary-dark px-4 py-2";

#[component]
pub fn H1(children: Children) -> impl IntoView {
    return view! {
        <h1 class=H1_CLASS>
            {children()}
        </h1>
    };
}

#[component]
pub fn H2(children: Children) -> impl IntoView {
    return view! {
        <h2 class=H2_CLASS>
            {children()}
        </h2>
    };
}

#[component]
pub fn H3(children: Children) -> impl IntoView {
    return view! {
        <h3 class=H3_CLASS>
            {children()}
        </h3>
    };
}

#[component]
pub fn H4(children: Children) -> impl IntoView {
    return view! {
        <h4 class=H4_CLASS>
            {children()}
        </h4>
    };
}

#[component]
pub fn P(children: Children) -> impl IntoView {
    return view! {
        <p class=NORMAL_CLASS>
            {children()}
        </p>
    };
}