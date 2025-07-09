use leptos::prelude::*;
use crate::components::atoms::typography::{H1};

/// 404 Not Found Page
#[component]
pub fn NotFound() -> impl IntoView {
    return view! { <H1>"Uh oh!" <br /> "We couldn't find that page!"</H1> };
}
