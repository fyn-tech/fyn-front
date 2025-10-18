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
 * filename: user_sign_in.rs
 * description: User sign-in form organism
 * ------------------------------------------------------------------------------------------------
 */

use leptos::{prelude::*, reactive::spawn_local};

use crate::common::size::*;
use crate::components::atoms::alert::*;
use crate::components::atoms::button::*;
use crate::components::atoms::layout::*;
use crate::components::molecules::form_field::*;
use crate::components::molecules::section::*;
use crate::domain::user_context::UserContext;
use crate::infrastructure::fyn_api_client::FynApiClient;

#[component]
pub fn UserSignIn() -> impl IntoView {
    let fyn_api_client = use_context::<FynApiClient>().expect("FynApiClient should be provided");
    let user_context =
        use_context::<RwSignal<Option<UserContext>>>().expect("User context should be provided");

    let username = RwSignal::new(String::new());
    let password = RwSignal::new(String::new());
    let loading = RwSignal::new(false);
    let error_message = RwSignal::new(None::<String>);

    let navigate = leptos_router::hooks::use_navigate();
    let handle_login = {
        let api_client = fyn_api_client.clone();
        let user_signal = user_context.clone();
        let loading_signal = loading.clone();
        let error_signal = error_message.clone();

        move || {
            let username_str = username.get();
            let password_str = password.get();

            if username_str.is_empty() || password_str.is_empty() {
                error_signal.set(Some("Username and password required".to_string()));
                return;
            }

            error_signal.set(None);
            loading_signal.set(true);

            let api_client = api_client.clone();
            let user_signal = user_signal.clone();
            let loading_signal = loading_signal.clone();
            let error_signal = error_signal.clone();
            let nav_fn = navigate.clone();

            spawn_local(async move {
                let response = api_client.login(username_str, password_str).await;

                match response {
                    Ok(user_data) => {
                        user_signal.set(Some(user_data));
                        nav_fn("/", Default::default());
                    }
                    Err(error) => {
                        error_signal.set(Some(format!("Login failed: {}", error)));
                    }
                }

                loading_signal.set(false);
            });
        }
    };

    return view! {
        <form on:submit=|e| e.prevent_default()>
            <Section level={SectionLevel::H2} centre={true} spaced={true} title={"Sign In".to_string()}>
                <Grid size={Size::Xl} cols=2>
                    <FormField
                        label={"Username".to_string()}
                        key={"username".to_string()}
                        placeholder={"username".to_string()}
                        input_type=InputType::Text { signal: username }
                    />
                    <FormField
                        label={"Password".to_string()}
                        key={"password".to_string()}
                        input_type=InputType::Password { signal: password }
                    />
                </Grid>
                <ErrorAlert message={error_message.read_only()} />
            </Section>
            <Stack align=FlexAlign::Center>
                <Button
                    button_data=ButtonData::new().text("Sign In").on_click(Box::new(handle_login))
                />
            </Stack>
        </form>
    };
}
