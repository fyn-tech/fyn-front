use leptos::prelude::*;

#[component]
pub fn H1(children: Children) -> impl IntoView {
    return view! {
        <h1 class="font-primary font-bold text-2xl text-content-primary dark:text-content-primary-dark">
            {children()}
        </h1>
    };
}

#[component]
pub fn H2(children: Children) -> impl IntoView {
    return view! {
        <h2 class="font-primary font-semibold text-xl text-content-primary dark:text-content-primary-dark">
            {children()}
        </h2>
    };
}

#[component]
pub fn H3(children: Children) -> impl IntoView {
    return view! {
        <h3 class="font-primary font-medium text-lg text-content-primary dark:text-content-primary-dark">
            {children()}
        </h3>
    };
}

#[component]
pub fn H4(children: Children) -> impl IntoView {
    return view! {
        <h4 class="font-primary font-medium text-base text-content-primary dark:text-content-primary-dark">
            {children()}
        </h4>
    };
}

#[component]
pub fn P(children: Children) -> impl IntoView {
    return view! {
        <h4 class="font-primary font-normal text-base text-justify text-content-primary dark:text-content-primary-dark">
            {children()}
        </h4>
    };
}