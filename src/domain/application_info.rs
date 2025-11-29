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
 * filename: application_info.rs
 * description: Domain entity representing the application information context
 * ------------------------------------------------------------------------------------------------
 */

use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
pub enum AppType {
    #[default]
    Unknown, // used as default (i.e. inert)
    PythonScript,
    LinuxBinary,
    WindowBinary,
    ShellScript,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct AppInfo {
    pub id: Uuid,
    pub name: String,
    pub app_type: AppType,
    pub file_path: String,
    pub schema_path: Option<String>,
    pub schema: Option<Value>,

    // Application definition
    pub executable_name: String,
    pub default_cli_args: String,
    pub use_mpi: bool,
}

#[allow(dead_code)]
impl AppInfo {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn id(mut self, id: Uuid) -> Self {
        self.id = id;
        self
    }

    pub fn name(mut self, name: String) -> Self {
        self.name = name;
        self
    }

    pub fn app_type(mut self, app_type: AppType) -> Self {
        self.app_type = app_type;
        self
    }

    pub fn file_path(mut self, file_path: String) -> Self {
        self.file_path = file_path;
        self
    }

    pub fn maybe_schema_path(mut self, schema_path: Option<String>) -> Self {
        self.schema_path = schema_path;
        self
    }

    pub fn schema_path(mut self, schema_path: String) -> Self {
        self.schema_path = Some(schema_path);
        self
    }

    pub fn maybe_schema(mut self, schema: Option<Value>) -> Self {
        self.schema = schema;
        self
    }

    pub fn schema(mut self, schema: Value) -> Self {
        self.schema = Some(schema);
        self
    }

    pub fn executable_name(mut self, executable_name: String) -> Self {
        self.executable_name = executable_name;
        self
    }

    pub fn default_cli_args(mut self, default_cli_args: String) -> Self {
        self.default_cli_args = default_cli_args;
        self
    }

    pub fn use_mpi(mut self, use_mpi: bool) -> Self {
        self.use_mpi = use_mpi;
        self
    }
}
