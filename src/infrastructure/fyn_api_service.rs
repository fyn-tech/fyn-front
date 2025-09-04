use fyn_api::apis::auth_api::auth_csrf_retrieve;
use fyn_api::apis::configuration::Configuration;

#[derive(Clone)]
pub struct FynApiService {
    config: Configuration,
}

impl FynApiService {
    pub fn new() -> Self {
        let mut config = Configuration::new();
        config.base_path = "http://localhost:8000".to_string(); // FIXME: is this needed?
        return Self { config };
    }

    pub async fn get_csrf_token(&self) -> Result<String, String> {
        let response = auth_csrf_retrieve(&self.config)
            .await
            .map_err(|e| format!("API error: {:?}", e))?;
        return response
            .csrf_token
            .ok_or_else(|| "Empty CSRF token from API".to_string());
    }
}
