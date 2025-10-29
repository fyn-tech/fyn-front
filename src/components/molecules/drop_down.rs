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
 * filename: drop_down.rs
 * description: Form field molecule component
 * ------------------------------------------------------------------------------------------------
 */
use leptos::prelude::*;

use crate::common::size::*;
use crate::components::atoms::layout::*;

#[component]
pub fn DropDown(trigger: impl IntoView + 'static, children: Children) -> impl IntoView {
    let show = RwSignal::new(false);
    let menu_content = children();

    view! {
        <Stack position={Position::Relative} fill_space={false} size={Size::None}>
            <div
                class="cursor-pointer"
                on:click=move |_| show.update(|open| *open = !*open)
            >
                {trigger}
            </div>
            <div
                class=move || format!("absolute top-full right-0 z-50 {}",
                    if show.get() { "" } else { "hidden" }
                )
            >
                <Stack
                    fill_space={false}
                    size={Size::Sm}
                    add_class={"bg-surface-700 shadow-lg rounded-lg p-2".to_string()}
                >
                    {menu_content}
                </Stack>
            </div>
        </Stack>
    }
}
