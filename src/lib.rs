mod common;
mod components;
mod pages;

use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{components::*, path};

// Top-Level pages
use crate::pages::home::Home;
use crate::pages::not_found::NotFound;
use crate::pages::register::Register;
use crate::pages::showcase::Showcase;

/// An app router which renders the homepage and handles 404's
#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Html attr:lang="en" attr:dir="ltr" attr:data-theme="light" />

        // sets the document title
        <Title text="Fyn-Tech" />

        // injects metadata in the <head> of the page
        <Meta charset="UTF-8" />
        <Meta name="viewport" content="width=device-width, initial-scale=1.0" />

        <div class="min-h-screen bg-surface-50 dark:bg-surface-950">
            <Router>
                <Routes fallback=NotFound >
                    <Route path=path!("/") view=Home />
                    <Route path=path!("/register") view=Register />
                    <Route path=path!("/showcase") view=Showcase />  // Add this
                </Routes>
            </Router>
        </div>
    }
}
