use leptos::prelude::*;

use crate::components::atoms::typography::{A, H1_CLASS};
use crate::components::organisms::navigation::*;

/// Default Home Page
#[component]
pub fn Home() -> impl IntoView {
    return view! {
        <Navigation/>
    };
}
