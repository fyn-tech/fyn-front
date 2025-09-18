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

use leptos::{prelude::*, reactive::spawn_local};

use crate::domain::user_context::UserContext;
use fyn_api::apis::accounts_api::accounts_users_create;
use fyn_api::apis::auth_api::auth_csrf_retrieve;
use fyn_api::apis::auth_api::auth_user_login_create;
use fyn_api::apis::configuration::Configuration;
use fyn_api::models::*;

#[derive(Clone)]
pub struct FynApiClient {
    config: Configuration,
    csrf_token: RwSignal<Option<String>>,
    session_token: RwSignal<Option<String>>,
    user_id: RwSignal<Option<String>>,
    loading: RwSignal<bool>,
}

impl FynApiClient {
    pub fn new() -> Self {
        let mut context = Self {
            config: Configuration::new(),
            csrf_token: RwSignal::new(None),
            session_token: RwSignal::new(None),
            user_id: RwSignal::new(None),
            loading: RwSignal::new(true),
        };
        context.config.base_path = "http://localhost:8000".to_string();

        spawn_local({
            let context = context.clone();
            async move {
                if let Err(e) = context.fetch_new_csrf_token().await {
                    leptos::logging::error!("Failed to fetch CSRF token: {:?}", e);
                    context.loading.set(false);
                }
            }
        });

        context
    }

    pub async fn fetch_new_csrf_token(&self) -> Result<(), String> {
        let response = auth_csrf_retrieve(&self.config)
            .await
            .map_err(|e| format!("API error: {:?}", e))?;

        self.csrf_token.set(Some(
            response
                .csrf_token
                .unwrap_or("Empty CSRF token from API".to_string()),
        ));

        self.loading.set(false);
        Ok(())
    }

    #[allow(dead_code)]
    pub fn get_token(&self) -> String {
        self.csrf_token.get().unwrap_or_else(|| "no token set".to_string())
    }

    pub async fn login(&self, username: String, password: String) -> Result<UserContext, String> {
        self.loading.set(true);

        let login_request = LoginRequest::new(username, password);

        let response = auth_user_login_create(&self.config, login_request)
            .await
            .map_err(|e| format!("API error: {:?}", e))?;

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

        self.loading.set(false);
        Ok(new_user)
    }

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

        let _response = accounts_users_create(&self.config, new_user_request)
            .await
            .map_err(|e| format!("API error: {:?}", e))?;

        self.loading.set(false);
        Ok("Created new user".to_string())
    }
}
