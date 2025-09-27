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

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct AppInfo {
    pub id: Uuid,
    pub name: String,
    pub file_path: String,
    pub schema_path: Option<String>,
    pub schema: Option<Value>,
}

impl AppInfo {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn new_basic(
        id: Uuid,
        name: String,
        file_path: String,
        schema_path: Option<String>,
    ) -> Self {
        Self {
            id,
            name,
            file_path,
            schema_path,
            schema: None,
        }
    }
}
