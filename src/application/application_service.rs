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
 * filename: application_service.rs
 * description: (Backend) Application service for interacting with available runnable applications
 * ------------------------------------------------------------------------------------------------
 */

use leptos::prelude::*;
use std::collections::HashMap;
use std::str::FromStr;
use uuid::Uuid;

use crate::domain::application_info::AppInfo;
use crate::domain::user_context::UserContext;
use crate::infrastructure::fyn_api_client::FynApiClient;

#[derive(Clone, Copy)]
pub struct AppService {
    api_client: FynApiClient,
    user_sig: RwSignal<Option<UserContext>>,
}

impl AppService {
    pub fn new() -> Self {
        Self {
            api_client: use_context::<FynApiClient>().expect("FynApiClient should be provided."),
            user_sig: use_context::<RwSignal<Option<UserContext>>>()
                .expect("UserContext should be provided."),
        }
    }

    pub fn get_application_list(self) -> LocalResource<Option<Vec<(String, String)>>> {
        LocalResource::new({
            move || async move {
                if self.user_sig.get().is_some() && self.user_sig.get().unwrap().apps.len() == 0 {
                    let app_info: Option<HashMap<Uuid, AppInfo>> =
                        match self.api_client.get_applications().await {
                            Ok(app_map) => Some(app_map),
                            Err(e) => {
                                leptos::logging::error!("Failed to get application data {:?}", e);
                                None
                            }
                        };
                    let user_cont = self.user_sig.get();
                    match user_cont {
                        Some(user) => {
                            self.user_sig.set(Some(user.maybe_apps(app_info)));
                        }
                        None => {}
                    }
                }
                Some(
                    self.user_sig
                        .get()
                        .unwrap_or_default()
                        .apps
                        .iter()
                        .map(|app| (app.0.to_string(), app.1.name.clone()))
                        .collect(),
                )
            }
        })
    }

    pub fn get_application_schema(
        self,
        application_id: RwSignal<String>,
    ) -> LocalResource<Option<String>> {
        LocalResource::new(move || {
            let app_id = application_id.get();
            async move {
                if app_id.is_empty() {
                    return None;
                }

                let app_selected_id = match Uuid::from_str(&app_id) {
                    Ok(ufs) => Some(ufs),
                    Err(_) => None,
                };

                if app_selected_id.is_some() && self.user_sig.get().is_some() {
                    let selected_app_id = app_selected_id.unwrap();
                    let user = self.user_sig.get().unwrap();

                    match user.apps.get(&selected_app_id) {
                        Some(app_info) => {
                            match &app_info.schema {
                                Some(existing_schema) => Some(existing_schema.to_string()), // existing schema, don't fetch
                                None => {
                                    // need to fetch new value
                                    let new_schema =
                                        match self.api_client.get_app_schema(selected_app_id).await
                                        {
                                            Some(schema) => schema,
                                            None => serde_json::Value::Null,
                                        };

                                    // Update the user context with the new schema
                                    let mut updated_user = user.clone();
                                    if let Some(app) = updated_user.apps.get_mut(&selected_app_id) {
                                        app.schema = Some(new_schema.clone());
                                    }
                                    self.user_sig.set(Some(updated_user));

                                    Some(new_schema.to_string())
                                }
                            }
                        }
                        None => None,
                    }
                } else {
                    None
                }
            }
        })
    }
}
