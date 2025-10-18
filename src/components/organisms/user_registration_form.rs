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
 * filename: user_registration_form.rs
 * description: User registration form organism
 * ------------------------------------------------------------------------------------------------
 */

use leptos::{prelude::*, reactive::spawn_local};
use leptos_router::hooks::use_navigate;

use crate::common::size::*;
use crate::components::atoms::alert::*;
use crate::components::atoms::button::*;
use crate::components::atoms::layout::*;
use crate::components::molecules::form_field::*;
use crate::components::molecules::section::*;
use crate::domain::user_context::UserContext;
use crate::infrastructure::fyn_api_client::FynApiClient;

#[derive(Clone)]
struct RegisterForm {
    first_name: RwSignal<String>,
    last_name: RwSignal<String>,
    username: RwSignal<String>,
    email: RwSignal<String>,
    password: RwSignal<String>,
    company: RwSignal<String>,
    country: RwSignal<String>,
    loading: RwSignal<bool>,
    error: RwSignal<Option<String>>,
}

impl RegisterForm {
    fn new() -> Self {
        Self {
            first_name: RwSignal::new(String::new()),
            last_name: RwSignal::new(String::new()),
            username: RwSignal::new(String::new()),
            email: RwSignal::new(String::new()),
            password: RwSignal::new(String::new()),
            company: RwSignal::new(String::new()),
            country: RwSignal::new(String::new()),
            loading: RwSignal::new(false),
            error: RwSignal::new(None),
        }
    }

    fn to_user_context(&self) -> UserContext {
        UserContext::new()
            .first_name(&self.first_name.get())
            .last_name(&self.last_name.get())
            .username(&self.username.get())
            .email(&self.email.get())
            .company(&self.company.get())
            .country(&self.country.get())
    }

    fn validate(&self) -> Result<(), String> {
        if self.username.get().is_empty() {
            return Err("Username is required".to_string());
        }
        if self.password.get().is_empty() {
            return Err("Password is required".to_string());
        }
        if self.email.get().is_empty() {
            return Err("Email is required".to_string());
        }
        if self.company.get().is_empty() {
            return Err("Company is required".to_string());
        }
        if self.country.get().is_empty() {
            return Err("Country is required".to_string());
        }
        Ok(())
    }

    fn set_error(&self, msg: String) {
        self.error.set(Some(msg));
        self.loading.set(false);
    }

    fn clear_error(&self) {
        self.error.set(None);
    }

    fn set_loading(&self, loading: bool) {
        self.loading.set(loading);
    }
}

#[component]
pub fn UserRegisterForm() -> impl IntoView {
    let fyn_api_client = use_context::<FynApiClient>().expect("FynApiClient should be provided");
    let reg_form = RegisterForm::new();
    let navigate = use_navigate();

    let handle_register = {
        let reg_form = reg_form.clone();
        move || {
            reg_form.clear_error();

            if let Err(error) = reg_form.validate() {
                reg_form.set_error(error);
                return;
            }

            reg_form.set_loading(true);

            let user_context = reg_form.to_user_context();
            let password = reg_form.password.get();

            let api_client = fyn_api_client.clone();
            let form = reg_form.clone();
            let nav_fn = navigate.clone();

            spawn_local(async move {
                let response = api_client.register(user_context, password).await;

                match response {
                    Ok(_) => {
                        nav_fn("/login", Default::default()); // Navigate to login after successful registration
                    }
                    Err(error) => {
                        form.set_error(format!("Registration failed: {}", error));
                    }
                }
            });
        }
    };

    view! {
        <form on:submit=|e| e.prevent_default()>
            <Section level={SectionLevel::H2} centre={true} spaced={true} title={"Register".to_string()}>
                <Grid size={Size::Xl} cols=2>
                    <FormField
                        label={"First Name".to_string()}
                        key={"first_name".to_string()}
                        placeholder={"first name".to_string()}
                        input_type=InputType::Text { signal: reg_form.first_name }
                    />
                    <FormField
                        label={"Last Name".to_string()}
                        key={"last_name".to_string()}
                        placeholder={"last name".to_string()}
                        input_type=InputType::Text { signal: reg_form.last_name }
                    />
                    <FormField
                        label={"Username".to_string()}
                        key={"username".to_string()}
                        placeholder={"username".to_string()}
                        input_type=InputType::Text { signal: reg_form.username }
                    />
                    <FormField
                        label={"Email".to_string()}
                        key={"email".to_string()}
                        input_type=InputType::Email { signal: reg_form.email }
                    />
                    <FormField
                        label={"Password".to_string()}
                        key={"password".to_string()}
                        input_type=InputType::Password { signal: reg_form.password }
                    />
                    <FormField
                        label={"Company".to_string()}
                        key={"company".to_string()}
                        placeholder={"company".to_string()}
                        input_type=InputType::Text { signal: reg_form.company }
                    />
                    <FormField
                        label={"Country".to_string()}
                        key={"country".to_string()}
                        placeholder={"country".to_string()}
                        input_type=InputType::Text { signal: reg_form.country }
                    />
                </Grid>
                <ErrorAlert message={reg_form.error.read_only()} />
            </Section>

            <Stack align=FlexAlign::Center>
                <Button button_data=ButtonData::new()
                .text("Create Account")
                .on_click(Box::new(move || handle_register()))
                />
            </Stack>
        </form>
    }
}
