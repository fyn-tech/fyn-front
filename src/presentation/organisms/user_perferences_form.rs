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

#[component]
pub fn UserPreferencesForm() -> impl IntoView {
    let fyn_api_client: FynApiClient =
        use_context::<FynApiClient>().expect("FynApiClient should be provided");
    let user_context =
        use_context::<RwSignal<Option<UserContext>>>().expect("UserContext should be provided.");
    let user_form = UserForm::from(user_context.get().unwrap_or_default());

    let handle_register = {
        let user_form = user_form.clone();
        move || {
            user_form.clear_error();
            user_form.set_loading(true);

            let user_context = user_context.clone();
            let user_form_context = UserContext::from(user_form.clone());

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
        }
    };

    view! {
        <form on:submit=|e| e.prevent_default()>
            <Section level={SectionLevel::H2} centre={true} spaced={true} title={"User Preferences".to_string()}>
                <Grid size={Size::Xl} cols=1>
                    <FormField
                        label={"Username".to_string()}
                        key={"username".to_string()}
                        placeholder={"username".to_string()}
                        input_type=InputType::Text { signal: user_form.username }
                    />
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
            </Section>

            <Stack align=FlexAlign::Center>
                <Button button_data=ButtonData::new()
                .text("Update")
                .on_click(Box::new(move || handle_register()))
                />
            </Stack>
        </form>
    }
}
