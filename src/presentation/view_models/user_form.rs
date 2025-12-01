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
 * filename: user_form.rs
 * description: View of the user context.
 * ------------------------------------------------------------------------------------------------
 */

use leptos::prelude::*;

use crate::domain::user_context::UserContext;

#[derive(Clone, Default)]
pub struct UserForm {
    // User Context Mirror Data
    pub first_name: RwSignal<String>,
    pub last_name: RwSignal<String>,
    pub username: RwSignal<String>,
    pub email: RwSignal<String>,
    pub password: RwSignal<String>,
    pub company: RwSignal<String>,
    pub country: RwSignal<String>,

    // Form Meta Data
    pub loading: RwSignal<bool>,
    pub error: RwSignal<Option<String>>,
}

#[warn(dead_code)]
impl UserForm {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn first_name(self, first_name: impl Into<String>) -> Self {
        self.first_name.set(first_name.into());
        self
    }

    pub fn maybe_first_name(self, first_name: Option<impl Into<String>>) -> Self {
        self.first_name
            .set(first_name.map(Into::into).unwrap_or_else(String::new));
        self
    }

    pub fn last_name(self, last_name: impl Into<String>) -> Self {
        self.last_name.set(last_name.into());
        self
    }

    pub fn maybe_last_name(self, last_name: Option<impl Into<String>>) -> Self {
        self.last_name
            .set(last_name.map(Into::into).unwrap_or_else(String::new));
        self
    }

    pub fn username(self, username: impl Into<String>) -> Self {
        self.username.set(username.into());
        self
    }

    pub fn maybe_username(self, username: Option<impl Into<String>>) -> Self {
        self.username
            .set(username.map(Into::into).unwrap_or_else(String::new));
        self
    }

    pub fn email(self, email: impl Into<String>) -> Self {
        self.email.set(email.into());
        self
    }

    pub fn maybe_email(self, email: Option<impl Into<String>>) -> Self {
        self.email
            .set(email.map(Into::into).unwrap_or_else(String::new));
        self
    }

    pub fn password(self, password: impl Into<String>) -> Self {
        self.password.set(password.into());
        self
    }

    pub fn maybe_password(self, password: Option<impl Into<String>>) -> Self {
        self.password
            .set(password.map(Into::into).unwrap_or_else(String::new));
        self
    }

    pub fn company(self, company: impl Into<String>) -> Self {
        self.company.set(company.into());
        self
    }

    pub fn maybe_company(self, company: Option<impl Into<String>>) -> Self {
        self.company
            .set(company.map(Into::into).unwrap_or_else(String::new));
        self
    }

    pub fn country(self, country: impl Into<String>) -> Self {
        self.country.set(country.into());
        self
    }

    pub fn maybe_country(self, country: Option<impl Into<String>>) -> Self {
        self.country
            .set(country.map(Into::into).unwrap_or_else(String::new));
        self
    }

    pub fn validate_new_user(&self) -> Result<(), String> {
        if self.username.get().is_empty() {
            return Err("Username is required".to_string());
        }
        if self.password.get().is_empty() {
            return Err("Password is required".to_string());
        }
        if self.email.get().is_empty() {
            return Err("Email is required".to_string());
        }
        if self.company.get().is_empty() {
            return Err("Company is required".to_string());
        }
        if self.country.get().is_empty() {
            return Err("Country is required".to_string());
        }
        Ok(())
    }

    pub fn set_error(&self, msg: String) {
        self.error.set(Some(msg));
        self.loading.set(false);
    }

    pub fn clear_error(&self) {
        self.error.set(None);
    }

    pub fn set_loading(&self, loading: bool) {
        self.loading.set(loading);
    }
}

impl From<UserContext> for UserForm {
    fn from(user: UserContext) -> UserForm {
        UserForm::new()
            .maybe_first_name(user.first_name)
            .maybe_last_name(user.last_name)
            .maybe_username(user.username)
            .maybe_email(user.email)
            .password("")
            .maybe_company(user.company)
            .maybe_country(user.country)
    }
}

impl From<&UserContext> for UserForm {
    fn from(user: &UserContext) -> UserForm {
        UserForm::new()
            .maybe_first_name(user.first_name.clone())
            .maybe_last_name(user.last_name.clone())
            .maybe_username(user.username.clone())
            .maybe_email(user.email.clone())
            .password("")
            .maybe_company(user.company.clone())
            .maybe_country(user.country.clone())
    }
}

impl From<UserForm> for UserContext {
    fn from(user: UserForm) -> UserContext {
        UserContext::new()
            .first_name(user.first_name.get())
            .last_name(user.last_name.get())
            .username(user.username.get())
            .email(user.email.get())
            .company(user.company.get())
            .country(user.country.get())
    }
}

impl From<&UserForm> for UserContext {
    fn from(user: &UserForm) -> UserContext {
        UserContext::new()
            .first_name(user.first_name.get().clone())
            .last_name(user.last_name.get().clone())
            .username(user.username.get().clone())
            .email(user.email.get().clone())
            .company(user.company.get().clone())
            .country(user.country.get().clone())
    }
}
