use leptos::prelude::*;

use crate::components::atoms::layout::*;
use crate::components::organisms::footer::*;
use crate::components::organisms::navigation::*;

#[component]
pub fn Standard(children: Children) -> impl IntoView {
    return view! {
        <Stack
            horizontal=false
            fill_space=true
            add_class="min-h-screen justify-between".to_string()
        >
            <Navigation/>
            <main class="flex-1 max-w-4xl mx-auto px-8 py-8 w-full">
                {children()}
            </main>
            <Footer/>
        </Stack>
    };
}
