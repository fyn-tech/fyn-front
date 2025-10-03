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
 * description: API client wrapper providing JWT authentication and communication with Fyn backend
 * ------------------------------------------------------------------------------------------------
 */

use std::collections::HashMap;

use chrono::{DateTime, Utc};
use fyn_api::apis::job_manager_api::job_manager_users_create;
use leptos::{prelude::*, reactive::spawn_local};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::application_info::AppInfo;
use crate::domain::job_context::{
    JobInfo as JobInfoDomain, JobStatus as JobStatusDomain,
    ResourceType,
};
use crate::domain::runner_info::{
    RunnerInfo as RunnerInfoDomain, RunnerState as RunnerStateDomain,
};
use crate::domain::user_context::UserContext;

use fyn_api::apis::accounts_api::{accounts_users_create, accounts_users_list};
use fyn_api::apis::application_registry_api::{
    application_registry_list, application_registry_program_schema_retrieve,
};
use fyn_api::apis::auth_api::auth_csrf_retrieve;
use fyn_api::apis::configuration::Configuration;
use fyn_api::apis::runner_manager_api::runner_manager_users_list;
use fyn_api::models::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TokenResponse {
    access: String,
    refresh: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TokenRefreshResponse {
    access: String,
}

#[derive(Clone)]
pub struct FynApiClient {
    config: RwSignal<Configuration>,
    access_token: RwSignal<Option<String>>,
    refresh_token: RwSignal<Option<String>>,
    csrf_token: RwSignal<Option<String>>,
    user_id: RwSignal<Option<String>>,
    loading: RwSignal<bool>,
}

impl FynApiClient {
    pub fn new() -> Self {
        let mut config = Configuration::new();
        config.base_path = "http://127.0.0.1:8000".to_string();

        let context = Self {
            config: RwSignal::new(config),
            access_token: RwSignal::new(None),
            refresh_token: RwSignal::new(None),
            csrf_token: RwSignal::new(None),
            user_id: RwSignal::new(None),
            loading: RwSignal::new(true),
        };

        // Optionally fetch CSRF token on init (if still using forms)
        spawn_local({
            let context = context.clone();
            async move {
                let _ = context.fetch_csrf_token().await;
                context.loading.set(false);
            }
        });

        context
    }

    // ---------------------------------------------------------------------------------------------
    // Authentication & Session Management
    // ---------------------------------------------------------------------------------------------

    pub async fn fetch_csrf_token(&self) -> Result<(), String> {
        leptos::logging::log!("Fetching CSRF token...");

        // CSRF still useful for forms, but NOT for API auth anymore
        let response = auth_csrf_retrieve(&self.config.get())
            .await
            .map_err(|e| format!("CSRF fetch error: {:?}", e))?;

        let csrf_token = response
            .csrf_token
            .unwrap_or("Empty CSRF token".to_string());

        leptos::logging::log!("CSRF token received: {}", csrf_token);
        self.csrf_token.set(Some(csrf_token));

        Ok(())
    }

    pub async fn login(&self, username: String, password: String) -> Result<UserContext, String> {
        self.loading.set(true);
        leptos::logging::log!("Attempting JWT login for user: {}", username);

        // Call Django's JWT token endpoint
        let response = reqwest::Client::new()
            .post(&format!("{}/api/token/", self.config.get().base_path))
            .json(&serde_json::json!({
                "username": username,
                "password": password
            }))
            .send()
            .await
            .map_err(|e| {
                leptos::logging::error!("Login request failed: {:?}", e);
                format!("Login failed: {:?}", e)
            })?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            leptos::logging::error!("Login failed with status: {}", error_text);
            self.loading.set(false);
            return Err(format!("Login failed: {}", error_text));
        }

        let tokens: TokenResponse = response.json().await
            .map_err(|e| {
                leptos::logging::error!("Token parse error: {:?}", e);
                self.loading.set(false);
                format!("Failed to parse tokens: {:?}", e)
            })?;

        leptos::logging::log!("JWT tokens received successfully");

        // Store tokens
        self.access_token.set(Some(tokens.access.clone()));
        self.refresh_token.set(Some(tokens.refresh.clone()));

        // Store tokens in localStorage for persistence
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                let _ = storage.set_item("access_token", &tokens.access);
                let _ = storage.set_item("refresh_token", &tokens.refresh);
            }
        }

        // Update OpenAPI client config with bearer token
        self.config.update(|c| {
            c.bearer_access_token = Some(tokens.access.clone());
        });

        // Fetch user details using the new token
        let user = self.get_current_user().await?;

        self.loading.set(false);
        Ok(user)
    }

    pub async fn refresh_access_token(&self) -> Result<(), String> {
        let refresh_token = self.refresh_token.get()
            .ok_or("No refresh token available")?;

        leptos::logging::log!("Refreshing access token...");

        let response = reqwest::Client::new()
            .post(&format!("{}/api/token/refresh/", self.config.get().base_path))
            .json(&serde_json::json!({
                "refresh": refresh_token
            }))
            .send()
            .await
            .map_err(|e| format!("Token refresh failed: {:?}", e))?;

        if !response.status().is_success() {
            return Err("Token refresh failed - please login again".to_string());
        }

        let new_token: TokenRefreshResponse = response.json().await
            .map_err(|e| format!("Failed to parse refreshed token: {:?}", e))?;

        // Update stored token
        self.access_token.set(Some(new_token.access.clone()));

        // Update localStorage
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                let _ = storage.set_item("access_token", &new_token.access);
            }
        }

        // Update OpenAPI client config
        self.config.update(|c| {
            c.bearer_access_token = Some(new_token.access.clone());
        });

        leptos::logging::log!("Access token refreshed successfully");
        Ok(())
    }

    pub async fn logout(&self) {
        leptos::logging::log!("Logging out...");

        // Clear tokens from memory
        self.access_token.set(None);
        self.refresh_token.set(None);
        self.user_id.set(None);

        // Clear from localStorage
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                let _ = storage.remove_item("access_token");
                let _ = storage.remove_item("refresh_token");
            }
        }

        // Clear from OpenAPI config
        self.config.update(|c| {
            c.bearer_access_token = None;
        });
    }

    async fn get_current_user(&self) -> Result<UserContext, String> {
        // Use the generated API client - it automatically adds the Bearer token!
        let users = accounts_users_list(&self.config.get())
            .await
            .map_err(|e| format!("Failed to get user: {:?}", e))?;

        let user = users.first()
            .ok_or("No user data returned")?;

        let mut context = UserContext::new();
        context.username = Some(user.username.clone());
        context.first_name = user.first_name.clone();
        context.last_name = user.last_name.clone();
        context.email = user.email.clone();
        context.company = Some(user.company.clone());
        context.country = Some(user.country.clone());

        Ok(context)
    }

    pub async fn restore_session(&self) -> Option<UserContext> {
        // Try to restore tokens from localStorage
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                if let (Ok(Some(access)), Ok(Some(refresh))) = (
                    storage.get_item("access_token"),
                    storage.get_item("refresh_token")
                ) {
                    leptos::logging::log!("Restoring session from localStorage");
                    self.access_token.set(Some(access.clone()));
                    self.refresh_token.set(Some(refresh));

                    // Update config
                    self.config.update(|c| {
                        c.bearer_access_token = Some(access);
                    });
                }
            }
        }

        // Check if we have tokens
        if self.access_token.get().is_none() {
            return None;
        }

        // Try to get user info
        match self.get_current_user().await {
            Ok(user) => {
                leptos::logging::log!("Session restored for: {:?}", user.username);
                Some(user)
            }
            Err(e) => {
                leptos::logging::log!("Session restore failed: {}", e);
                // Token might be expired, try refresh
                if self.refresh_access_token().await.is_ok() {
                    // Retry getting user
                    self.get_current_user().await.ok()
                } else {
                    // Clear invalid tokens
                    self.logout().await;
                    None
                }
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
        Ok("User created successfully".to_string())
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
                leptos::logging::error!("Application registry schema fetch error: {}", e);
                None::<serde_json::Value>
            }
        }
    }

    // ---------------------------------------------------------------------------------------------
    // Job
    // ---------------------------------------------------------------------------------------------

    pub async fn submit_new_job(&self, new_job: JobInfoDomain) -> Option<JobInfoDomain> {
        let mut job_request = JobInfoRequest::new(new_job.application_id);
        job_request.name = Some(new_job.name.clone());
        job_request.priority = Some(new_job.priority as i32);
        job_request.command_line_args = Some(new_job.command_line_args.clone());
        job_request.status = Some(domain_api_job_status(new_job.status));
        job_request.assigned_runner = Some(Some(new_job.runner_id));

        let response = job_manager_users_create(&self.config.get(), job_request).await;

        match response {
            Ok(job_info) => {
                let mut updated_new_job = new_job;
                match updated_new_job.set_id(job_info.id) {
                    Ok(_) => {}
                    Err(e) => {
                        leptos::logging::error!("Error setting job id {:?}", e);
                        return None;
                    }
                };
                Some(updated_new_job)
            }
            Err(e) => {
                leptos::logging::error!("job_manager_users_create failed: {:?}", e);
                None
            }
        }
    }

    /// Upload a web File as a job resource (for browser-based uploads)
    ///
    /// NOTE: This bypasses the generated OpenAPI client because:
    /// 1. The generated client uses reqwest which doesn't support File uploads in WASM
    /// 2. The backend expects multipart/form-data, not JSON
    /// 3. Browser File objects can't be converted to PathBuf
    pub async fn upload_job_resource_file(
        &self,
        job_id: Uuid,
        file: web_sys::File,
        resource_type: &str,
        description: Option<&str>,
    ) -> Result<(), String> {
        use gloo_net::http::Request;

        let csrf_token = self.get_csrf_token().unwrap_or_default();
        let access_token = self.access_token.get().unwrap_or_default();
        let base_url = &self.config.get().base_path;

        let form_data = web_sys::FormData::new()
            .map_err(|_| "Failed to create FormData".to_string())?;

        form_data
            .append_with_str("job", &job_id.to_string())
            .map_err(|_| "Failed to append job".to_string())?;

        form_data
            .append_with_str("resource_type", resource_type)
            .map_err(|_| "Failed to append resource_type".to_string())?;

        form_data
            .append_with_blob("file", &file)
            .map_err(|_| "Failed to append file".to_string())?;

        if let Some(desc) = description {
            form_data
                .append_with_str("description", desc)
                .map_err(|_| "Failed to append description".to_string())?;
        }

        let response = Request::post(&format!("{}/job_manager/resources/users/", base_url))
            .header("Authorization", &format!("Bearer {}", access_token))
            .header("X-CSRFToken", &csrf_token)
            .body(form_data)
            .map_err(|e| format!("Failed to build request: {:?}", e))?
            .send()
            .await
            .map_err(|e| format!("Request failed: {:?}", e))?;

        if response.ok() {
            Ok(())
        } else {
            Err(format!("Upload failed with status: {}", response.status()))
        }
    }

    // ---------------------------------------------------------------------------------------------
    // Utility Methods
    // ---------------------------------------------------------------------------------------------

    /// Get CSRF token from browser cookies (for Django CSRF protection on forms)
    fn get_csrf_token(&self) -> Option<String> {
        use wasm_bindgen::JsCast;

        let window = web_sys::window()?;
        let document = window.document()?;
        let html_document = document.dyn_into::<web_sys::HtmlDocument>().ok()?;
        let cookie = html_document.cookie().ok()?;

        cookie.split(';').find_map(|c| {
            let parts: Vec<&str> = c.trim().splitn(2, '=').collect();
            if parts.len() == 2 && parts[0] == "csrftoken" {
                Some(parts[1].to_string())
            } else {
                None
            }
        })
    }

    // ---------------------------------------------------------------------------------------------
    // Runners
    // ---------------------------------------------------------------------------------------------

    pub async fn get_runner_info(&self) -> Result<HashMap<Uuid, RunnerInfoDomain>, String> {
        self.loading.set(true);

        leptos::logging::log!("Fetching runner info with JWT auth...");

        // The bearer_access_token in config automatically adds:
        // Authorization: Bearer <token>
        let response = runner_manager_users_list(&self.config.get())
            .await
            .map_err(|e| {
                leptos::logging::error!("Runner info API error: {:?}", e);
                format!("API error: {:?}", e)
            })?;

        leptos::logging::log!("Runner info retrieved successfully");
        self.loading.set(false);

        let runner_infos = response
            .iter()
            .map(|run| {
                (
                    run.id,
                    RunnerInfoDomain::new_complete(
                        run.id,
                        run.name.as_ref().unwrap().to_string(),
                        api_domain_runner_state(run.state.unwrap()),
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

// -------------------------------------------------------------------------------------------------
// Front-Back End Enum Mapping
// -------------------------------------------------------------------------------------------------

#[allow(dead_code)]
fn domain_api_job_status(domain_status: JobStatusDomain) -> StatusEnum {
    match domain_status {
        JobStatusDomain::UploadingInputResources => StatusEnum::Ui,
        JobStatusDomain::Queued => StatusEnum::Qd,
        JobStatusDomain::Preparing => StatusEnum::Pr,
        JobStatusDomain::FetchingResources => StatusEnum::Fr,
        JobStatusDomain::Starting => StatusEnum::St,
        JobStatusDomain::Running => StatusEnum::Rn,
        JobStatusDomain::Paused => StatusEnum::Pd,
        JobStatusDomain::CleaningUp => StatusEnum::Cu,
        JobStatusDomain::UploadingResults => StatusEnum::Ur,
        JobStatusDomain::Succeeded => StatusEnum::Sd,
        JobStatusDomain::Failed => StatusEnum::Fd,
        JobStatusDomain::FailedResourceError => StatusEnum::Fs,
        JobStatusDomain::FailedTerminated => StatusEnum::Fm,
        JobStatusDomain::FailedTimeout => StatusEnum::Fo,
        JobStatusDomain::FailedRunnerException => StatusEnum::Fe,
    }
}

#[allow(dead_code)]
fn api_domain_job_status(api_status: StatusEnum) -> JobStatusDomain {
    match api_status {
        StatusEnum::Ui => JobStatusDomain::UploadingInputResources,
        StatusEnum::Qd => JobStatusDomain::Queued,
        StatusEnum::Pr => JobStatusDomain::Preparing,
        StatusEnum::Fr => JobStatusDomain::FetchingResources,
        StatusEnum::St => JobStatusDomain::Starting,
        StatusEnum::Rn => JobStatusDomain::Running,
        StatusEnum::Pd => JobStatusDomain::Paused,
        StatusEnum::Cu => JobStatusDomain::CleaningUp,
        StatusEnum::Ur => JobStatusDomain::UploadingResults,
        StatusEnum::Sd => JobStatusDomain::Succeeded,
        StatusEnum::Fd => JobStatusDomain::Failed,
        StatusEnum::Fs => JobStatusDomain::FailedResourceError,
        StatusEnum::Fm => JobStatusDomain::FailedTerminated,
        StatusEnum::Fo => JobStatusDomain::FailedTimeout,
        StatusEnum::Fe => JobStatusDomain::FailedRunnerException,
    }
}

#[allow(dead_code)]
fn domain_api_resource_type(domain_status: ResourceType) -> ResourceTypeEnum {
    match domain_status {
        ResourceType::Input => ResourceTypeEnum::In,
        ResourceType::Output => ResourceTypeEnum::Out,
        ResourceType::Config => ResourceTypeEnum::Cfg,
        ResourceType::Log => ResourceTypeEnum::Log,
        ResourceType::Temp => ResourceTypeEnum::Tmp,
        ResourceType::Result => ResourceTypeEnum::Res,
    }
}

#[allow(dead_code)]
fn api_domain_resource_type(resource_type: ResourceTypeEnum) -> ResourceType {
    match resource_type {
        ResourceTypeEnum::In => ResourceType::Input,
        ResourceTypeEnum::Out => ResourceType::Output,
        ResourceTypeEnum::Cfg => ResourceType::Config,
        ResourceTypeEnum::Log => ResourceType::Log,
        ResourceTypeEnum::Tmp => ResourceType::Temp,
        ResourceTypeEnum::Res => ResourceType::Result,
    }
}

#[allow(dead_code)]
fn domain_api_runner_state(domain_state: RunnerStateDomain) -> StateEnum {
    match domain_state {
        RunnerStateDomain::Idle => StateEnum::Id,
        RunnerStateDomain::Busy => StateEnum::Bs,
        RunnerStateDomain::Offline => StateEnum::Of,
        RunnerStateDomain::Unregistered => StateEnum::Ur,
        RunnerStateDomain::Unknown => StateEnum::Of, // Map Unknown to Offline
    }
}

fn api_domain_runner_state(api_state: StateEnum) -> RunnerStateDomain {
    match api_state {
        StateEnum::Id => RunnerStateDomain::Idle,
        StateEnum::Bs => RunnerStateDomain::Busy,
        StateEnum::Of => RunnerStateDomain::Offline,
        StateEnum::Ur => RunnerStateDomain::Unregistered,
    }
}
