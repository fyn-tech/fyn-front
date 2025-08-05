use leptos::prelude::*;

use crate::components::organisms::user_registration_form::*;
use crate::templates::standard::*;

#[component]
pub fn Register() -> impl IntoView {
    return view! {
      <Standard>
        <UserRegisterForm/>
      </Standard>
    };
}
