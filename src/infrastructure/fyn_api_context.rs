use crate::infrastructure::fyn_api_service::FynApiService;
use leptos::{prelude::*, reactive::spawn_local};

#[derive(Clone)]
pub struct FynApiSessionContext {
    service: FynApiService,
    csrf_token: RwSignal<Option<String>>,
    session_token: RwSignal<Option<String>>,
    user_id: RwSignal<Option<String>>,
    loading: RwSignal<bool>,
}

impl FynApiSessionContext {
    pub fn new() -> Self {
        let context = Self {
            service: FynApiService::new(),
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
                }
            }
        });

        return context;
    }

    pub async fn fetch_new_csrf_token(&self) -> Result<(), String> {
        let response = self.service.get_csrf_token().await?;
        self.csrf_token.set(Some(response));
        self.loading.set(false);
        return Ok(());
    }

    pub fn get_token(&self) -> String {
        return self.csrf_token.get().unwrap_or("no token set".to_string());
    }
}
