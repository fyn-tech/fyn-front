use leptos::prelude::*;

use crate::components::organisms::fyn_tech_profile::FynTechProfile;
use crate::components::templates::standard::Standard;

#[component]
pub fn Home() -> impl IntoView {
    return view! {
        <Standard>
            <FynTechProfile />
        </Standard>
    };
}
