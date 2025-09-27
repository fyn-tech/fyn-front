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
 * filename: runner_info.rs
 * description: Domain entity representing the runner context
 * ------------------------------------------------------------------------------------------------
 */

use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Clone, Debug, Default)]
pub enum RunnerState {
    Idle,
    Busy,
    Offline,
    Unregistered,
    #[default]
    Unknown,
}

#[derive(Clone, Debug, Default)]
pub struct RunnerInfo {
    pub id: Uuid,
    pub name: String,
    pub state: RunnerState,
    pub created_at: DateTime<Utc>,
    pub last_contact: Option<DateTime<Utc>>,
}

impl RunnerInfo {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn new_complete(
        id: Uuid,
        name: String,
        state: RunnerState,
        created_at: DateTime<Utc>,
        last_contact: Option<DateTime<Utc>>,
    ) -> Self {
        Self {
            id,
            name,
            state,
            created_at,
            last_contact,
        }
    }
}
