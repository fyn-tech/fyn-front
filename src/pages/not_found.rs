use leptos::prelude::*;

use crate::components::atoms::layout::*;
use crate::components::atoms::typography::{A, H1, H1_CLASS};

/// 404 Not Found Page
#[component]
pub fn NotFound() -> impl IntoView {
    return view! {
            <Stack horizontal=false
                align=FlexAlign::Center
                add_class="min-h-screen justify-center".to_string()>
                <H1>"Uh oh!"</H1>
                <H1>"We couldn't find that page!"</H1>
                <A href={"/".to_string()} text_class={H1_CLASS.to_string()}>"Back home"</A>
            </Stack>
    };
}
