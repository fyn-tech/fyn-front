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

use fyn_api::models::JobInfo;
use std::collections::HashMap;
use uuid::Uuid;

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
    pub apps: HashMap<Uuid, AppInfo>,

    // runners related
    pub runners: Option<HashMap<Uuid, RunnerInfo>>,

    // job related
    pub jobs: Option<HashMap<Uuid, JobInfo>>,
}

#[allow(dead_code)]
impl UserContext {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn username(mut self, username: impl Into<String>) -> Self {
        self.username = Some(username.into());
        self
    }

    pub fn first_name(mut self, first_name: impl Into<String>) -> Self {
        self.first_name = Some(first_name.into());
        self
    }

    pub fn maybe_first_name(mut self, first_name: Option<impl Into<String>>) -> Self {
        self.first_name = first_name.map(Into::into);
        self
    }

    pub fn last_name(mut self, last_name: impl Into<String>) -> Self {
        self.last_name = Some(last_name.into());
        self
    }

    pub fn maybe_last_name(mut self, last_name: Option<impl Into<String>>) -> Self {
        self.last_name = last_name.map(Into::into);
        self
    }

    pub fn email(mut self, email: impl Into<String>) -> Self {
        self.email = Some(email.into());
        self
    }

    pub fn maybe_email(mut self, email: Option<impl Into<String>>) -> Self {
        self.email = email.map(Into::into);
        self
    }

    pub fn company(mut self, company: impl Into<String>) -> Self {
        self.company = Some(company.into());
        self
    }

    pub fn maybe_company(mut self, company: Option<impl Into<String>>) -> Self {
        self.company = company.map(Into::into);
        self
    }

    pub fn country(mut self, country: impl Into<String>) -> Self {
        self.country = Some(country.into());
        self
    }

    pub fn maybe_country(mut self, country: Option<impl Into<String>>) -> Self {
        self.country = country.map(Into::into);
        self
    }
}
