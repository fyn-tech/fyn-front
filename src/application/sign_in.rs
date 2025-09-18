use leptos::prelude::*;

use crate::components::organisms::user_sign_in::*;
use crate::components::templates::standard::*;

#[component]
pub fn SignIn() -> impl IntoView {
    return view! {
      <Standard>
        <UserSignIn/>
      </Standard>
    };
}
