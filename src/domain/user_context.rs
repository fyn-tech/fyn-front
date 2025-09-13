#[derive(Clone)]
pub struct UserContext {
    pub username: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub company: Option<String>,
    pub country: Option<String>,
}

impl UserContext {
    pub fn new() -> UserContext {
        UserContext {
            username: None,
            first_name: None,
            last_name: None,
            email: None,
            company: None,
            country: None,
        }
    }

    pub fn new_partial(
        username: Option<String>,
        first_name: Option<String>,
        last_name: Option<String>,
    ) -> UserContext {
        UserContext {
            username: username,
            first_name: first_name,
            last_name: last_name,
            email: None,
            company: None,
            country: None,
        }
    }
}
