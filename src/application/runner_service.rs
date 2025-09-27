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
 * filename: runner_service.rs
 * description: API client wrapper providing authentication and communication with Fyn backend
 * ------------------------------------------------------------------------------------------------
 */

use leptos::prelude::*;
use std::collections::HashMap;
use uuid::Uuid;

use crate::domain::runner_info::RunnerInfo;
use crate::domain::user_context::UserContext;
use crate::infrastructure::fyn_api_client::FynApiClient;

pub struct RunnerService {
    api_client: FynApiClient,
    user_sig: RwSignal<Option<UserContext>>,
}

impl RunnerService {
    pub fn new() -> Self {
        Self {
            api_client: use_context::<FynApiClient>().expect("FynApiClient should be provided."),
            user_sig: use_context::<RwSignal<Option<UserContext>>>()
                .expect("UserContext should be provided."),
        }
    }

    fn is_runner_list_cached(&self) -> bool {
        match self.user_sig.get() {
            Some(user) => {
                match user.runners {
                    Some(_) => true, // have a user and list
                    None => false,   // we have a user, but norunners
                }
            }
            None => false, // no users, no cache
        }
    }

    async fn fetch_and_update_runner_list(&self) {
        if self.user_sig.get().is_none() {
            return;
        }

        let response = self.api_client.get_runner_info().await;

        match response {
            Ok(runners) => {
                let runners_len = runners.len();
                // Update the signal properly
                self.user_sig.update(|user_opt| {
                    if let Some(user) = user_opt {
                        user.runners = Some(runners);
                    }
                });
                leptos::logging::log!("Successfully updated runners: {} found", runners_len);
            }
            Err(error) => {
                leptos::logging::error!("Failed fetch runners: {}", error);
            }
        }
    }

    pub fn get_runners(force_update: bool) -> LocalResource<Option<HashMap<Uuid, RunnerInfo>>> {
        LocalResource::new({
            move || async move {
                let service = RunnerService::new();
                if service.user_sig.get().is_none() {
                    leptos::logging::log!("No user context available");
                    return None;
                }
                if force_update || !service.is_runner_list_cached() {
                    leptos::logging::log!("Fetching runners (force: {}, cached: {})", force_update, service.is_runner_list_cached());
                    service.fetch_and_update_runner_list().await;
                }
                let runners = service.user_sig.get().unwrap().runners.clone();
                leptos::logging::log!("Returning runners: {:?}", runners.as_ref().map(|r| r.len()));
                runners
            }
        })
    }
}
