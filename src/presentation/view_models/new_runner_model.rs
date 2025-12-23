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
 * filename: new_runner_model.rs
 * description: Model view for new runner creation.
 * ------------------------------------------------------------------------------------------------
 */

use crate::domain::runner_info::*;
use leptos::prelude::*;

#[derive(Clone, Default)]
pub struct NewRunnerModel {
    // Required Context Mirror Data
    pub name: RwSignal<String>,

    // Form Meta Data
    pub loading: RwSignal<bool>,
    pub error: RwSignal<Option<String>>,
    pub info: RwSignal<Option<String>>,
}

#[allow(dead_code)]
impl NewRunnerModel {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn name(self, name: impl Into<String>) -> Self {
        self.name.set(name.into());
        self
    }

    pub fn maybe_name(self, name: Option<impl Into<String>>) -> Self {
        self.name
            .set(name.map(Into::into).unwrap_or_else(String::new));
        self
    }

    pub fn validate_new_user(&self) -> Result<(), String> {
        if self.name.get().is_empty() {
            return Err("Name is required".to_string());
        }
        Ok(())
    }

    pub fn set_error(&self, msg: String) {
        self.error.set(Some(msg));
        self.loading.set(false);
    }

    pub fn set_info(&self, msg: String) {
        self.info.set(Some(msg));
        self.loading.set(false);
    }

    pub fn clear_error(&self) {
        self.error.set(None);
    }

    pub fn clear_info(&self) {
        self.info.set(None);
    }

    pub fn set_loading(&self, loading: bool) {
        self.loading.set(loading);
    }
}

impl From<NewRunnerModel> for RunnerInfo {
    fn from(form: NewRunnerModel) -> Self {
        RunnerInfo::new().name(form.name.get())
    }
}
