use leptos::prelude::*;
use serde::{Deserialize, Serialize};

use crate::common::size::*;
use crate::components::atoms::button::*;
use crate::components::atoms::input::*;
use crate::components::atoms::layout::*;
use crate::components::atoms::typography::{H3, H4_CLASS, NORMAL_CLASS};

#[component]
pub fn FormField() -> impl IntoView {
    return view! {
      <Stack horizontal=true add_class="justify-between items-center".to_string()>
        <label>"Test"</label>
        <Text id="test".to_string() key="test".to_string() />
      </Stack>
    };
}
