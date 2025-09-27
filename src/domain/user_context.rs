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
 * filename: user_context.rs
 * description: Domain entity representing user authentication context and user data
 * ------------------------------------------------------------------------------------------------
 */

use fyn_api::models::App;

use crate::domain::application_info::*;
use crate::domain::runner_info::*;

#[derive(Clone, Debug, Default)]
pub struct UserContext {
    pub username: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub company: Option<String>,
    pub country: Option<String>,

    // application related
    pub apps: Vec<AppInfo>,

    // runners related
    pub runners: Vec<RunnerInfo>,
}

impl UserContext {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn new_partial(
        username: Option<String>,
        first_name: Option<String>,
        last_name: Option<String>,
    ) -> Self {
        Self {
            username,
            first_name,
            last_name,
            email: None,
            company: None,
            country: None,
            apps: Vec::new(),
            runners: Vec::new(),
        }
    }
}
