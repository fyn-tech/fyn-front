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
 * filename: user_preferences_form.rs
 * description: User preferences form organism
 * ------------------------------------------------------------------------------------------------
 */

use leptos::{prelude::*, reactive::spawn_local};

use crate::common::size::*;
use crate::domain::user_context::UserContext;
use crate::infrastructure::fyn_api_client::FynApiClient;
use crate::presentation::atoms::alert::*;
use crate::presentation::atoms::button::*;
use crate::presentation::atoms::layout::*;
use crate::presentation::molecules::form_field::*;
use crate::presentation::molecules::section::*;
use crate::presentation::view_models::user_form::*;

fn handle_password_update(
    current_password: RwSignal<String>,
    password_0: RwSignal<String>,
    password_1: RwSignal<String>,
    error_message: RwSignal<Option<String>>,
) {
    error_message.set(None);

    if password_0.get() != password_1.get() {
        error_message.set(Some(String::from("New passwords do not match")));
        return;
    }

    let fyn_api_client: FynApiClient =
        use_context::<FynApiClient>().expect("FynApiClient should be provided");
    let user_context =
        use_context::<RwSignal<Option<UserContext>>>().expect("UserContext should be provided.");
    let cloned_message = error_message.clone();
    let navigate = leptos_router::hooks::use_navigate();

    spawn_local(async move {
        let response = fyn_api_client
            .update_user_password(&current_password.get(), &password_0.get())
            .await;
        let nav_fn = navigate.clone();
        match response {
            Ok(()) => {
                let _logout_response = fyn_api_client.logout().await;
                user_context.set(None);
                nav_fn("/sign_in", Default::default());
            }
            Err(error) => {
                cloned_message.set(Some(format!("Update failed: {}", error)));
            }
        }
    });
}

fn handle_details_update(user_form: &UserForm) {
    let fyn_api_client: FynApiClient =
        use_context::<FynApiClient>().expect("FynApiClient should be provided");
    let user_context =
        use_context::<RwSignal<Option<UserContext>>>().expect("UserContext should be provided.");
    user_form.clear_error();
    user_form.set_loading(true);

    let user_context = user_context.clone();
    let user_form_context = UserContext::from(user_form);

    let api_client = fyn_api_client.clone();
    let form = user_form.clone();

    spawn_local(async move {
        let response = api_client.update_user(user_form_context).await;

        match response {
            Ok(updated_user) => {
                user_context.set(Some(updated_user));
            }
            Err(error) => {
                form.set_error(format!("Update failed: {}", error));
            }
        }
    });

    user_form.set_loading(false);
}

#[component]
pub fn UserPreferencesForm() -> impl IntoView {
    let user_context =
        use_context::<RwSignal<Option<UserContext>>>().expect("UserContext should be provided.");
    let user_form = UserForm::from(user_context.get().unwrap_or_default());

    let password = RwSignal::new(String::new());
    let new_password_0 = RwSignal::new(String::new());
    let new_password_1 = RwSignal::new(String::new());
    let password_error = RwSignal::new(None);

    view! {
        <form on:submit=|e| e.prevent_default()>
            <Section level={SectionLevel::H2} centre=true spaced=true is_first=true max_width=Size::Xl4 title={"User Details".to_string()}>
                <Grid size={Size::Xl} cols=1>
                    <FormField
                        label={"First Name".to_string()}
                        key={"first_name".to_string()}
                        placeholder={"first name".to_string()}
                        input_type=InputType::Text { signal: user_form.first_name }
                    />
                    <FormField
                        label={"Last Name".to_string()}
                        key={"last_name".to_string()}
                        placeholder={"last name".to_string()}
                        input_type=InputType::Text { signal: user_form.last_name }
                    />
                    <FormField
                        label={"Email".to_string()}
                        key={"email".to_string()}
                        input_type=InputType::Email { signal: user_form.email }
                    />
                    <FormField
                        label={"Company".to_string()}
                        key={"company".to_string()}
                        placeholder={"company".to_string()}
                        input_type=InputType::Text { signal: user_form.company }
                    />
                    <FormField
                        label={"Country".to_string()}
                        key={"country".to_string()}
                        placeholder={"country".to_string()}
                        input_type=InputType::Text { signal: user_form.country }
                    />
                </Grid>
                <ErrorAlert message={user_form.error.read_only()} />
                <Stack align=FlexAlign::Center size=Size::Md add_class="py-4".to_string()>
                    <Button button_data=ButtonData::new()
                    .text("Update Details")
                    .on_click(Box::new(move || handle_details_update(&user_form)))
                    />
                </Stack>
            </Section>

            <Section level={SectionLevel::H2} centre={true} spaced={true} max_width=Size::Xl4 title={"Account".to_string()}>
                <Grid size={Size::Xl} cols=1>
                    <FormField
                        label={"Current Password".to_string()}
                        key={"password".to_string()}
                        input_type=InputType::Password { signal: password }
                    />
                    <FormField
                        label={"New Password".to_string()}
                        key={"new_password_0".to_string()}
                        input_type=InputType::Password  { signal: new_password_0 }
                    /><FormField
                        label={"Repeat New Password".to_string()}
                        key={"new_password_1".to_string()}
                        input_type=InputType::Password  { signal: new_password_1 }
                    />
                </Grid>
                <ErrorAlert message={password_error.read_only()} />
                <Stack align=FlexAlign::Center size=Size::Md add_class="py-4".to_string()>
                    <Button button_data=ButtonData::new()
                    .text("Update Account")
                    .on_click(Box::new(move || handle_password_update(password, new_password_0, new_password_1, password_error)))
                    />
                </Stack>
            </Section>


        </form>
    }
}
