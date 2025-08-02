use leptos::prelude::*;
use serde::{Deserialize, Serialize};

use crate::common::size::*;
use crate::components::atoms::button::*;
use crate::components::atoms::input::*;
use crate::components::atoms::layout::*;
use crate::components::atoms::typography::{Size, H3, H4_CLASS, NORMAL_CLASS};

#[component]
pub fn FormField() -> impl IntoView {
    let class_str = format!("{} {}", H4_CLASS, "");
    return view! {
      <Stack horizontal=true align=FlexAlign::Center add_class="justify-between".to_string()>
        <label class={H4_CLASS}>"Test"</label>
        <Text id="test".to_string() key="test".to_string() />
      </Stack>
    };
}
