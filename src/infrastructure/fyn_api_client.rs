use leptos::server_fn::response;
use leptos::{prelude::*, reactive::spawn_local};

use fyn_api::apis::auth_api::auth_csrf_retrieve;
use fyn_api::apis::auth_api::auth_user_login_create;
use fyn_api::apis::configuration::Configuration;
use fyn_api::models::*;

use crate::domain::user_service::UserService;

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

        return context;
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
        return Ok(());
    }

    pub fn get_token(&self) -> String {
        return self.csrf_token.get().unwrap_or("no token set".to_string());
    }

    pub async fn login(&self, username: String, password: String) -> Result<UserService, String> {
        self.loading.set(true);

        let login_request = LoginRequest::new(username, password);

        let response = auth_user_login_create(&self.config, login_request)
            .await
            .map_err(|e| format!("API error: {:?}", e))?;

        let fetched_user_data = response
            .user_data
            .unwrap_or(Box::new(AuthUserLoginCreate200ResponseUserData::new()));

        let new_user = UserService::new_partial(
            fetched_user_data.username,
            fetched_user_data.first_name,
            fetched_user_data.last_name,
        );

        self.user_id.set(fetched_user_data.id);
        self.session_token.set(response.token);

        self.loading.set(false);
        return Ok(new_user);
    }
}
