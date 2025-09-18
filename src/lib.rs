mod application;
mod common;
mod components;
mod domain;
mod infrastructure;

use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{components::*, path};

use crate::domain::user_context::UserContext;
use crate::infrastructure::fyn_api_client::FynApiClient;

// Top-Level pages
use crate::application::home::Home;
use crate::application::not_found::NotFound;
use crate::application::register::Register;
use crate::application::showcase::Showcase;
use crate::application::sign_in::SignIn;
use crate::application::simulate::Simulate;

/// An app router which renders the homepage and handles 404's
#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    // Fetch CSRF token on app startup
    let fyn_api_client_context = FynApiClient::new();
    provide_context(fyn_api_client_context.clone());

    let user_context: RwSignal<Option<UserContext>> = RwSignal::new(None);
    provide_context(user_context);

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
                    <Route path=path!("/simulate") view=Simulate />
                    <Route path=path!("/register") view=Register />
                    <Route path=path!("/sign_in") view=SignIn />
                    <Route path=path!("/showcase") view=Showcase />  // Add this
                </Routes>
            </Router>
        </div>
    }
}
