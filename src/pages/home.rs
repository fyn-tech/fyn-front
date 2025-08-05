use leptos::prelude::*;

use crate::components::organisms::navigation::*;

/// Default Home Page
#[component]
pub fn Home() -> impl IntoView {
    return view! {
        <Navigation/>
    };
}
