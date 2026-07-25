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
 * filename: standard.rs
 * description: Standard page template component
 * ------------------------------------------------------------------------------------------------
 */

use leptos::prelude::*;

use crate::common::size::*;
use crate::presentation::atoms::layout::*;
use crate::presentation::organisms::footer::*;
use crate::presentation::organisms::navigation::*;

#[component]
pub fn Standard(children: Children) -> impl IntoView {
    return view! {
        <Stack
            horizontal=false
            fill_space=true
            size=Size::Xl4
            add_class="min-h-screen".to_string()
        >
            <Navigation/>
                <Stack fill_space=false size=Size::None add_class="flex-grow".to_string()>
                    {children()}
                </Stack>
            <Footer/>
        </Stack>
    };
}
