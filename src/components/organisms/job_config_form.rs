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
 * filename: job_config_form.rs
 * description: Job configuration form organism
 * ------------------------------------------------------------------------------------------------
 */
use leptos::{prelude::*, reactive::spawn_local};
use serde_json::json;
use std::collections::HashMap;
use std::str::FromStr;
use uuid::Uuid;

use crate::common::size::*;
use crate::components::atoms::button::*;
use crate::components::atoms::layout::*;
use crate::components::atoms::typography::*;
use crate::components::molecules::form_field::*;
use crate::components::molecules::schema_form::{SchemaForm, SchemaFormState};
use crate::components::molecules::section::*;
use crate::domain::application_info::AppInfo;
use crate::domain::runner_info::RunnerInfo;
use crate::domain::user_context::UserContext;
use crate::domain::job_context::*;
use crate::infrastructure::fyn_api_client::FynApiClient;

fn get_application_list() -> LocalResource<Option<Vec<(String, String)>>> {
    LocalResource::new({
        move || async move {
            let fyn_api_client =
                use_context::<FynApiClient>().expect("FynApiClient should be provided");
            let user_context =
                use_context::<RwSignal<Option<UserContext>>>().expect("user should be provided");
            if user_context.get().is_some() && user_context.get().unwrap().apps.len() == 0 {
                let app_info: Option<HashMap<Uuid, AppInfo>> =
                    fyn_api_client.get_applications().await;
                let user_cont = user_context.get();
                match user_cont {
                    Some(mut user) => {
                        user.apps = app_info.unwrap_or_default();
                        user_context.set(Some(user));
                    }
                    None => {}
                }
            }
            Some(
                user_context
                    .get()
                    .unwrap_or_default()
                    .apps
                    .iter()
                    .map(|app| (app.0.to_string(), app.1.name.clone()))
                    .collect(),
            )
        }
    })
}

fn get_application_schema(application_id: RwSignal<String>) -> LocalResource<Option<String>> {
    LocalResource::new(move || {
        let app_id = application_id.get();
        async move {
            let fyn_api_client =
                use_context::<FynApiClient>().expect("FynApiClient should be provided");

            let user_context = use_context::<RwSignal<Option<UserContext>>>()
                .expect("UserContext should be provided.");

            if app_id.is_empty() {
                return None;
            }

            let app_selected_id = match Uuid::from_str(&app_id) {
                Ok(ufs) => Some(ufs),
                Err(_) => None,
            };

            if app_selected_id.is_some() && user_context.get().is_some() {
                let selected_app_id = app_selected_id.unwrap();
                let user = user_context.get().unwrap();

                match user.apps.get(&selected_app_id) {
                    Some(app_info) => {
                        match &app_info.schema {
                            Some(existing_schema) => Some(existing_schema.to_string()), // existing schema, don't fetch
                            None => {
                                // need to fetch new value
                                let new_schema =
                                    match fyn_api_client.get_app_schema(selected_app_id).await {
                                        Some(schema) => schema,
                                        None => serde_json::Value::Null,
                                    };

                                // Update the user context with the new schema
                                let mut updated_user = user.clone();
                                if let Some(app) = updated_user.apps.get_mut(&selected_app_id) {
                                    app.schema = Some(new_schema.clone());
                                }
                                user_context.set(Some(updated_user));

                                Some(new_schema.to_string())
                            }
                        }
                    }
                    None => None,
                }
            } else {
                None
            }
        }
    })
}

#[component]
pub fn JobConfigForm(runner_list: Option<HashMap<Uuid, RunnerInfo>>) -> impl IntoView {
    let job_name = RwSignal::new(String::new());
    let job_priority = RwSignal::new(Some(0i64));
    let application_id = RwSignal::new(String::new());
    let runner_id = RwSignal::new(String::new());
    let application_list = get_application_list();
    let fetch_json_schema = get_application_schema(application_id);

    // Signal to receive form data from SchemaForm
    let schema_form_state: RwSignal<Option<SchemaFormState>> = RwSignal::new(None);

    // Clone for closure
    let runner_list_clone = runner_list.clone();

    // Button state signal (created outside so it persists across renders)
    let button_state_signal = RwSignal::new(State::Default);

    // Button click handler
    let on_submit_click = move || {
        let fyn_api_client =
            use_context::<FynApiClient>().expect("FynApiClient should be provided");
        let clone2_state = button_state_signal.clone();

        spawn_local(async move {
            // Helper for UUID parsing
            let parse_uuid = |s: &str, field: &str| -> Result<Uuid, String> {
                Uuid::from_str(s).map_err(|e| format!("Invalid {}: {:?}", field, e))
            };
            clone2_state.set(State::Loading);

            let runner_uuid = match parse_uuid(&runner_id.get(), "runner UUID") {
                Ok(id) => id,
                Err(e) => {
                    leptos::logging::error!("{}", e);
                    clone2_state.set(State::Error);
                    return;
                }
            };

            let app_uuid = match parse_uuid(&application_id.get(), "application UUID") {
                Ok(id) => id,
                Err(e) => {
                    leptos::logging::error!("{}", e);
                    clone2_state.set(State::Error);
                    return;
                }
            };

            let new_job_request = match JobInfo::new_request(
                job_name.get(),
                app_uuid,
                Some(runner_uuid),
                job_priority.get().unwrap_or(0),
                "executable".to_string(),
                Some(json!(["arg1", "arg2", "arg3"])),
            ) {
                Ok(job) => job,
                Err(e) => {
                    leptos::logging::error!("Failed to create job: {:?}", e);
                    clone2_state.set(State::Error);
                    return;
                }
            };

            let mut created_job = match fyn_api_client.submit_new_job(&new_job_request).await {
                Ok(job) => {
                    leptos::logging::log!("Job created: {:?}", job.id);
                    job
                }
                Err(e) => {
                    leptos::logging::error!("Failed to submit new job: {}", e);
                    clone2_state.set(State::Error);
                    return;
                }
            };

            // Upload config file if we have form data
            if let Some(form_state) = schema_form_state.get() {
                let config_json = form_state.to_json();
                leptos::logging::log!("Creating config file from form data");

                match FynApiClient::create_json_file(&config_json, "config_file.json") {
                    Ok(config_file) => {
                        leptos::logging::log!("Uploading config file for job {}", created_job.id);

                        match fyn_api_client
                            .upload_job_resource_file(
                                created_job.id,
                                config_file,
                                "CFG", // CONFIG resource type
                                Some("Application configuration file"),
                            )
                            .await
                        {
                            Ok(_) => {
                                leptos::logging::log!("Config file uploaded successfully");
                            }
                            Err(e) => {
                                leptos::logging::error!("Failed to upload config file: {}", e);
                                clone2_state.set(State::Error);
                            }
                        }
                    }
                    Err(e) => {
                        leptos::logging::error!("Failed to create config file: {}", e);
                        clone2_state.set(State::Error);
                    }
                }
            } else {
                leptos::logging::log!("No form data to upload");
            }

            // place into queued state to trigger runner pick up
            created_job.status = JobStatus::Queued;
            match fyn_api_client.patch_job(&created_job).await {
                Ok(job_info) => {
                    leptos::logging::log!("Job {} ({}) set to queued", job_info.name, job_info.id);
                }
                Err(e) => {
                    leptos::logging::error!(
                        "Error setting job {} ({}) to queue: {}",
                        created_job.name,
                        created_job.id,
                        e
                    );
                    clone2_state.set(State::Error);
                    return;
                }
            };
            clone2_state.set(State::Success);
        });
    };

    return view! {
        <div class=format!("w-max {} h-full overflow-y-auto", padding(Size::Md))>
            <Section level={SectionLevel::H2} centre={false} spaced={false} title={"Application Selection".to_string()}>
            // meta data
            {move || {
                let options = application_list.get()
                    .flatten()
                    .unwrap_or(vec![("...".to_string(), "Loading...".to_string())]);
                view! {
                    <FormField
                        label={"Job Name".to_string()}
                        key={"job_name".to_string()}
                        placeholder={"name".to_string()}
                        input_type=InputType::Text {
                            signal: job_name
                        }
                    />
                    <FormField
                        label={"Job Priority".to_string()}
                        key={"job_priority".to_string()}
                        placeholder={"0".to_string()}
                        input_type=InputType::Integer { signal: job_priority, min: Some(0), max: Some(100), step: Some(1) }
                    />
                    <FormField
                        label={"Application".to_string()}
                        key={"application".to_string()}
                        input_type=InputType::SelectText {
                            options: options,
                            signal: application_id
                        }
                    />
                }
            }}
            </Section>

            // actual input data collection
            <Section level={SectionLevel::H2} centre={false} spaced={false} title={"Application Setup".to_string()}>
            {move || {
               match fetch_json_schema.get() {
                   Some(Some(value)) => view! {
                      <SchemaForm
                        schema_json=value.to_string()
                        key=application_id.get()
                        form_state_out=schema_form_state
                      />

                      // Runner selection
                      {
                          let runner_options = match &runner_list_clone {
                              Some(runners) => {
                                  runners.iter().map(|(id, runner)| {
                                      (id.to_string(), runner.name.clone())
                                  }).collect::<Vec<(String, String)>>()
                              },
                              None => vec![("...".to_string(), "Loading runners...".to_string())]
                          };

                          view! {
                              <FormField
                                  label={"Runner".to_string()}
                                  key={"runner".to_string()}
                                  input_type=InputType::SelectText {
                                      options: runner_options,
                                      signal: runner_id
                                  }
                              />
                          }
                      }

                      <Stack align=FlexAlign::Center>
                          <Button button_data={
                              let mut btn = ButtonData::new()
                                  .text("Submit")
                                  .size(Size::Md)
                                  .on_click(on_submit_click);
                              btn.state_signal = button_state_signal;
                              btn
                          } />
                      </Stack>
                    }.into_any(),
                    Some(None) | None => view!{<P>"select an application..."</P>}.into_any()
               }
             }
            }
            </Section>


        </div>
    };
}
