use leptos::prelude::*;

#[component]
pub fn H1(children: Children) -> impl IntoView {
    return view! {
        <h1 class="text-2xl font-primary font-bold text-content-primary dark:text-content-primary-dark">
            {children()}
        </h1>
    };
}

#[component]
pub fn H2(children: Children) -> impl IntoView {
    return view! {
        <h2 class="text-xl font-primary font-semibold text-content-primary dark:text-content-primary-dark">
            {children()}
        </h2>
    };
}

#[component]
pub fn H3(children: Children) -> impl IntoView {
    return view! {
        <h3 class="text-lg font-primary font-medium text-content-primary dark:text-content-primary-dark">
            {children()}
        </h3>
    };
}

#[component]
pub fn H4(children: Children) -> impl IntoView {
    return view! {
        <h4 class="text-base font-primary font-medium text-content-primary dark:text-content-primary-dark">
            {children()}
        </h4>
    };
}

#[component]
pub fn P(children: Children) -> impl IntoView {
    return view! {
        <h4 class="text-base font-primary font-normal text-content-primary dark:text-content-primary-dark">
            {children()}
        </h4>
    };
}