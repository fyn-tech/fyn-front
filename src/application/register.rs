use leptos::prelude::*;

use crate::components::organisms::user_registration_form::*;
use crate::components::templates::standard::*;

#[component]
pub fn Register() -> impl IntoView {
    return view! {
      <Standard>
        <UserRegisterForm/>
      </Standard>
    };
}
