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
 * filename: runner_adapter.rs
 * description: Adapter for backend runner models and domain runner models
 * ------------------------------------------------------------------------------------------------
 */

use std::time::UNIX_EPOCH;

use fyn_api::models::{
    runner_info::RunnerInfo, runner_info_full::RunnerInfoFull,
    runner_info_request::RunnerInfoRequest, state_enum::StateEnum,
};
use uuid::Uuid;

use crate::domain::runner_info::{RunnerInfo as RunnerInfoDomain, RunnerState};

impl From<Option<StateEnum>> for RunnerState {
    fn from(api: Option<StateEnum>) -> Self {
        match api {
            Some(_api) => match _api {
                StateEnum::Id => RunnerState::Idle,
                StateEnum::Bs => RunnerState::Busy,
                StateEnum::Of => RunnerState::Offline,
                StateEnum::Ur => RunnerState::Unregistered,
            },
            None => RunnerState::Unknown,
        }
    }
}

impl From<RunnerState> for Option<StateEnum> {
    fn from(domain: RunnerState) -> Self {
        match domain {
            RunnerState::Idle => Some(StateEnum::Id),
            RunnerState::Busy => Some(StateEnum::Bs),
            RunnerState::Offline => Some(StateEnum::Of),
            RunnerState::Unregistered => Some(StateEnum::Ur),
            RunnerState::Unknown => None,
        }
    }
}

impl From<RunnerInfo> for RunnerInfoDomain {
    fn from(api: RunnerInfo) -> Self {
        RunnerInfoDomain::new()
            .id(api.id)
            .name(api.name)
            .state(api.state.into())
            .owner(api.owner)
            .created_at(match api.created_at.parse() {
                Ok(date) => date,
                Err(error) => {
                    leptos::logging::error!(
                        "Failed to convert date time for 'created_at' {:?}.",
                        error
                    );
                    UNIX_EPOCH.into()
                }
            })
            .maybe_last_contact(match api.last_contact {
                Some(str_date) => match str_date.parse() {
                    Ok(date) => Some(date),
                    Err(error) => {
                        leptos::logging::error!(
                            "Failed to convert date time for 'last_contact' {:?}.",
                            error
                        );
                        Some(UNIX_EPOCH.into())
                    }
                },
                None => None,
            })
    }
}

impl From<RunnerInfoDomain> for RunnerInfo {
    fn from(domain: RunnerInfoDomain) -> Self {
        RunnerInfo {
            id: (domain.id),
            name: (domain.name),
            state: (domain.state.into()),
            owner: (domain.owner),
            created_at: (domain.created_at.to_string()),
            last_contact: (match domain.last_contact {
                Some(date_time) => Some(date_time.to_string()),
                None => None,
            }),
        }
    }
}

impl From<RunnerInfoFull> for RunnerInfoDomain {
    fn from(api: RunnerInfoFull) -> Self {
        RunnerInfoDomain::new()
            .id(api.id)
            .name(api.name)
            .state(api.state.into())
            .owner(api.owner)
            .token(api.token.clone())
            .created_at(match api.created_at.parse() {
                Ok(date) => date,
                Err(error) => {
                    leptos::logging::error!(
                        "Failed to convert date time for 'created_at' {:?}.",
                        error
                    );
                    UNIX_EPOCH.into()
                }
            })
            .maybe_last_contact(match api.last_contact {
                Some(str_date) => match str_date.parse() {
                    Ok(date) => Some(date),
                    Err(error) => {
                        leptos::logging::error!(
                            "Failed to convert date time for 'last_contact' {:?}.",
                            error
                        );
                        Some(UNIX_EPOCH.into())
                    }
                },
                None => None,
            })
    }
}

impl From<RunnerInfoRequest> for RunnerInfoDomain {
    fn from(api: RunnerInfoRequest) -> Self {
        RunnerInfoDomain::new()
            .name(api.name)
            .state(api.state.into())
    }
}

impl From<RunnerInfoDomain> for RunnerInfoRequest {
    fn from(domain: RunnerInfoDomain) -> Self {
        RunnerInfoRequest {
            name: domain.name,
            state: domain.state.into(),
        }
    }
}
