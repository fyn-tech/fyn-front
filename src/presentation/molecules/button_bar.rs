/* ------------------------------------------------------------------------------------------------
 * Fyn-Front: Modern CFD/CAE Web Interface
 * Copyright (C) 2025 Fyn-Front Authors
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see <https://www.gnu.org/licenses/>.
 * ------------------------------------------------------------------------------------------------
 * filename: button_bar.rs
 * description: Button bar molecule component
 * ------------------------------------------------------------------------------------------------
 */

use leptos::prelude::*;

use crate::common::size::*;
use crate::presentation::atoms::button::*;
use crate::presentation::atoms::layout::*;

#[component]
pub fn ButtonBar(
    #[prop(default = Variant::Primary)] variant: Variant,
    #[prop(default = true)] horizontal: bool,
    items: Vec<impl IntoView + 'static>,
) -> impl IntoView {
    let class_str = format!(
        "overflow-hidden  justify-self-start {} {}",
        ROUND_BORDER,
        variant.base_colour()
    );
    let dividers = format!(
        "bg-surface-300 dark:bg-surface-700 {}",
        if horizontal { "w-px" } else { "h-px" },
    );

    return view! {
      <Stack size={Size::None} horizontal={horizontal} fill_space={false} add_class={class_str}>{
        items.into_iter().enumerate().map(|(i, item)| {
          view!{
            {if i > 0 {
              view! {<div class={dividers.clone()}></div> }.into_any()
            }
            else {
              view! { }.into_any()
            }}
            {item}
          }
        }).collect::<Vec<_>>()
      }
      </Stack>
    };
}
