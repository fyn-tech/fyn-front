use leptos::prelude::*;

#[derive(Debug, Clone)]
pub struct AuthState {
    pub csrf_token: Option<String>,
    pub is_authenticated: bool,
    pub user_id: Option<String>,
    pub loading: bool,
}

impl Default for AuthState {
    fn default() -> Self {
        Self {
            csrf_token: None,
            is_authenticated: false,
            user_id: None,
            loading: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct AuthContext {
    pub state: Signal<AuthState>,
    pub set_csrf_token: WriteSignal<Option<String>>,
    pub set_authenticated: WriteSignal<bool>,
    pub set_loading: WriteSignal<bool>,
}

pub fn provide_auth_context() -> AuthContext {
    let (csrf_token, set_csrf_token) = signal(None::<String>);
    let (is_authenticated, set_authenticated) = signal(false);
    let (loading, set_loading) = signal(true);

    let state = Signal::derive(move || AuthState {
        csrf_token: csrf_token.get(),
        is_authenticated: is_authenticated.get(),
        user_id: None, // Will add user fetching later
        loading: loading.get(),
    });

    let context = AuthContext {
        state: state.into(),
        set_csrf_token,
        set_authenticated,
        set_loading,
    };

    provide_context(context.clone());
    context
}

pub fn use_auth_context() -> AuthContext {
    expect_context::<AuthContext>()
}
