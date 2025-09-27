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
 * filename: fyn_api_client.rs
 * description: API client wrapper providing authentication and communication with Fyn backend
 * ------------------------------------------------------------------------------------------------
 */

use std::collections::HashMap;

use chrono::{DateTime, Utc};
use fyn_api::apis::job_manager_api::job_manager_resources_users_create;
use leptos::{prelude::*, reactive::spawn_local};
use serde_json::Value;
use uuid::Uuid;

use crate::domain::application_info::AppInfo;
use crate::domain::runner_info::{
    RunnerInfo as RunnerInfoDomain, RunnerState as RunnerStateDomain,
};
use crate::domain::user_context::UserContext;
use fyn_api::apis::accounts_api::{accounts_users_create, accounts_users_list};
use fyn_api::apis::application_registry_api::{
    application_registry_list, application_registry_program_schema_retrieve,
};
use fyn_api::apis::auth_api::auth_csrf_retrieve;
use fyn_api::apis::auth_api::auth_user_login_create;
use fyn_api::apis::configuration::Configuration;
use fyn_api::apis::runner_manager_api::runner_manager_users_list;
use fyn_api::models::*;

#[derive(Clone)]
pub struct FynApiClient {
    config: RwSignal<Configuration>,
    csrf_token: RwSignal<Option<String>>,
    session_token: RwSignal<Option<String>>,
    user_id: RwSignal<Option<String>>,
    loading: RwSignal<bool>,
}

impl FynApiClient {
    pub fn new() -> Self {
        let mut config = Configuration::new();
        config.base_path = "http://127.0.0.1:8000".to_string();

        // In WASM, cookies are handled automatically by the browser
        // The reqwest client will automatically include cookies for same-origin requests

        let context = Self {
            config: RwSignal::new(config),
            csrf_token: RwSignal::new(None),
            session_token: RwSignal::new(None),
            user_id: RwSignal::new(None),
            loading: RwSignal::new(true),
        };

        spawn_local({
            let context = context.clone();
            async move {
                if let Err(e) = context.fetch_new_csrf_token().await {
                    leptos::logging::error!("Failed to fetch CSRF token: {:?}", e);
                    context.loading.set(false);
                    return;
                }
            }
        });

        context
    }

    // ---------------------------------------------------------------------------------------------
    // Authentication & Session Management
    // ---------------------------------------------------------------------------------------------

    pub async fn fetch_new_csrf_token(&self) -> Result<(), String> {
        leptos::logging::log!("Fetching CSRF token...");
        let response = auth_csrf_retrieve(&self.config.get())
            .await
            .map_err(|e| format!("API error: {:?}", e))?;

        let csrf_token = response
            .csrf_token
            .unwrap_or("Empty CSRF token from API".to_string());

        leptos::logging::log!("CSRF token received: {}", csrf_token);
        self.csrf_token.set(Some(csrf_token));

        self.loading.set(false);
        Ok(())
    }

    #[allow(dead_code)]
    pub fn get_token(&self) -> String {
        self.csrf_token
            .get()
            .unwrap_or_else(|| "no token set".to_string())
    }

    pub async fn login(&self, username: String, password: String) -> Result<UserContext, String> {
        self.loading.set(true);
        leptos::logging::log!("Attempting login for user: {}", username);

        let login_request = LoginRequest::new(username, password);

        let response = auth_user_login_create(&self.config.get(), login_request)
            .await
            .map_err(|e| {
                leptos::logging::error!("Login API error: {:?}", e);
                format!("API error: {:?}", e)
            })?;

        leptos::logging::log!("Login response received: {:?}", response.status);

        let fetched_user_data = response
            .user_data
            .unwrap_or(Box::new(AuthUserLoginCreate200ResponseUserData::new()));

        let new_user = UserContext::new_partial(
            fetched_user_data.username,
            fetched_user_data.first_name,
            fetched_user_data.last_name,
        );

        self.user_id.set(fetched_user_data.id);
        self.session_token.set(response.token);
        leptos::logging::log!("Login successful, session should be established");
        self.loading.set(false);
        Ok(new_user)
    }

    pub async fn restore_session(&self) -> Option<UserContext> {
        match accounts_users_list(&self.config.get()).await {
            Ok(response_) => {
                let mut new_context = UserContext::new();
                response_.first().map(|ret_user| {
                    new_context.username = Some(ret_user.username.clone());
                    new_context.first_name = ret_user.first_name.clone();
                    new_context.last_name = ret_user.last_name.clone();
                    new_context.email = ret_user.email.clone();
                    new_context.company = Some(ret_user.company.clone());
                    new_context.country = Some(ret_user.country.clone());
                });
                leptos::logging::log!(
                    "restored session for {}",
                    new_context.username.clone().unwrap()
                );
                Some(new_context)
            }
            Err(_) => {
                leptos::logging::log!("No valid session");
                None
            }
        }
    }

    // ---------------------------------------------------------------------------------------------
    // User/Accounts
    // ---------------------------------------------------------------------------------------------

    pub async fn register(
        &self,
        new_user: UserContext,
        password: String,
    ) -> Result<String, String> {
        self.loading.set(true);

        let mut new_user_request = UserRequest::new(
            new_user.username.unwrap(),
            password,
            new_user.country.unwrap(),
            new_user.company.unwrap(),
        );
        new_user_request.first_name = new_user.first_name;
        new_user_request.last_name = new_user.last_name;
        new_user_request.email = new_user.email;

        let _response = accounts_users_create(&self.config.get(), new_user_request)
            .await
            .map_err(|e| format!("API error: {:?}", e))?;

        self.loading.set(false);
        Ok("Created new user".to_string())
    }

    // ---------------------------------------------------------------------------------------------
    // Applications
    // ---------------------------------------------------------------------------------------------

    pub async fn get_applications(&self) -> Option<HashMap<Uuid, AppInfo>> {
        match application_registry_list(&self.config.get()).await {
            Ok(list_of_apps) => Some(
                list_of_apps
                    .iter()
                    .map(|app| {
                        (
                            app.id,
                            AppInfo::new_basic(
                                app.id,
                                app.name.clone(),
                                app.file_path.clone(),
                                app.schema_path.clone(),
                            ),
                        )
                    })
                    .collect(),
            ),
            Err(e) => {
                leptos::logging::error!("Application registry API error: {:?}", e);
                None
            }
        }
    }

    pub async fn get_app_schema(&self, app_id: Uuid) -> Option<serde_json::Value> {
        match application_registry_program_schema_retrieve(&self.config.get(), &app_id.to_string())
            .await
        {
            Ok(json_string) => Some(json_string),
            Err(e) => {
                leptos::logging::error!("Application registry schema fecth error: {}", e);
                None::<serde_json::Value>
            }
        }
    }

    // ---------------------------------------------------------------------------------------------
    // Job
    // ---------------------------------------------------------------------------------------------

    // pub async fn submit_new_job(&self) -> Option<serde_json::Value> {
    //     // job_manager_resources_users_create(configuration, job_resource_request)
    // }

    // ---------------------------------------------------------------------------------------------
    // Runners
    // ---------------------------------------------------------------------------------------------

    pub async fn get_runner_info(&self) -> Result<HashMap<Uuid, RunnerInfoDomain>, String> {
        self.loading.set(true);

        leptos::logging::log!("Making authenticated request for runner info...");

        // The sessionid cookie will automatically be sent with this request
        let _response = runner_manager_users_list(&self.config.get())
            .await
            .map_err(|e| {
                leptos::logging::error!("Runner info API error: {:?}", e);
                format!("API error: {:?}", e)
            })?;

        leptos::logging::log!("Runner info response received successfully");
        self.loading.set(false);

        let runner_infos = _response
            .iter()
            .map(|run| {
                (
                    run.id,
                    RunnerInfoDomain::new_complete(
                        run.id,
                        run.name.as_ref().unwrap().to_string(),
                        match run.state {
                            Some(StateEnum::Id) => RunnerStateDomain::Idle,
                            Some(StateEnum::Bs) => RunnerStateDomain::Busy,
                            Some(StateEnum::Of) => RunnerStateDomain::Offline,
                            Some(StateEnum::Ur) => RunnerStateDomain::Unregistered,
                            None => RunnerStateDomain::Unknown,
                        },
                        run.created_at.parse::<DateTime<Utc>>().unwrap(),
                        run.last_contact
                            .as_ref()
                            .flatten()
                            .and_then(|s| s.parse::<DateTime<Utc>>().ok()),
                    ),
                )
            })
            .collect::<HashMap<Uuid, RunnerInfoDomain>>();

        Ok(runner_infos)
    }
}
