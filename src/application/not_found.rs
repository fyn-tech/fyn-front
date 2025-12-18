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
 * filename: not_found.rs
 * description: 404 Not Found page component
 * ------------------------------------------------------------------------------------------------
 */


use leptos::prelude::*;

use crate::presentation::atoms::layout::*;
use crate::presentation::atoms::typography::{A, H1, H1_CLASS};

/// 404 Not Found Page
#[component]
pub fn NotFound() -> impl IntoView {
    return view! {
            <Stack horizontal=false
                align=FlexAlign::Center
                add_class="min-h-screen justify-center".to_string()>
                <H1>"Uh oh!"</H1>
                <H1>"We couldn't find that page!"</H1>
                <A href={"/".to_string()} text_class={H1_CLASS.to_string()}>"Back home"</A>
            </Stack>
    };
}
