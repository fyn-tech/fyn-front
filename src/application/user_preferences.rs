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
 * filename: user_preferences.rs
 * description: User registration page component
 * ------------------------------------------------------------------------------------------------
 */

use leptos::prelude::*;

use crate::components::organisms::user_perferences_form::*;
use crate::components::templates::standard::*;

#[component]
pub fn UserPreferences() -> impl IntoView {
    return view! {
      <Standard>
        <UserPreferencesForm/>
      </Standard>
    };
}
