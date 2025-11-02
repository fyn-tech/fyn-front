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
use crate::components::atoms::alert::*;
use crate::components::atoms::button::*;
use crate::components::atoms::layout::*;
use crate::components::molecules::form_field::*;
use crate::components::molecules::schema_form::{SchemaForm, SchemaFormState};
use crate::components::molecules::section::*;
use crate::domain::job_context::*;
use crate::domain::runner_info::RunnerInfo;
use crate::domain::user_context::UserContext;
use crate::infrastructure::fyn_api_client::FynApiClient;

// -------------------------------------------------------------------------------------------------
// Helpers
// -------------------------------------------------------------------------------------------------

#[derive(Clone, Copy)]
pub struct ApplicationSchemaConfig {
    pub id: RwSignal<String>,
    pub list: LocalResource<Option<Vec<(String, String)>>>,
    pub schema: LocalResource<Option<String>>,
}

// -------------------------------------------------------------------------------------------------
// Component
// -------------------------------------------------------------------------------------------------

#[component]
pub fn JobConfigForm(
    runner_list: Option<HashMap<Uuid, RunnerInfo>>,
    applications: ApplicationSchemaConfig,
) -> impl IntoView {
    let job_name = RwSignal::new(String::new());
    let job_priority = RwSignal::new(Some(0i64));
    // let application_id = RwSignal::new(String::new());
    let runner_id = RwSignal::new(String::new());
    //let application_list = get_application_list();
    //let fetch_json_schema = get_application_schema(application_id);
    let error_message = RwSignal::new(None::<String>);

    // Signal to receive form data from SchemaForm
    let schema_form_state: RwSignal<Option<SchemaFormState>> = RwSignal::new(None);

    // Clone for closure
    let runner_list_clone = runner_list.clone();

    // Button state signal
    let button_state_signal = RwSignal::new(State::Default);
    let button_text_signal = RwSignal::new("Submit".to_string());
    let on_submit_click = move || {
        let fyn_api_client =
            use_context::<FynApiClient>().expect("FynApiClient should be provided");
        let cl_button_state_signal = button_state_signal.clone();
        let cl_button_text_signal = button_text_signal.clone();

        spawn_local(async move {
            // Helper for UUID parsing
            let parse_uuid = |s: &str, field: &str| -> Result<Uuid, String> {
                Uuid::from_str(s).map_err(|e| format!("Invalid {}: {:?}", field, e))
            };
            cl_button_state_signal.set(State::Loading);
            let error_signal = error_message.clone();
            error_signal.set(None);

            let runner_uuid = match parse_uuid(&runner_id.get(), "runner UUID") {
                Ok(id) => id,
                Err(e) => {
                    error_signal.set(Some(format!("{}", e)));
                    cl_button_state_signal.set(State::Error);
                    return;
                }
            };

            let app_uuid = match parse_uuid(&applications.id.get(), "application UUID") {
                Ok(id) => id,
                Err(e) => {
                    error_signal.set(Some(format!("{}", e)));
                    cl_button_state_signal.set(State::Error);
                    return;
                }
            };

            let new_job_request = match JobInfo::new()
                .name(job_name.get())
                .application_id(app_uuid)
                .runner_id(runner_uuid)
                .priority(job_priority.get().unwrap_or(0))
                .executable("executable")
                .command_line_args(&json!(["arg1", "arg2", "arg3"]))
                .build()
            {
                Ok(job) => job,
                Err(e) => {
                    error_signal.set(Some(format!("Failed to create job: {:?}", e)));
                    cl_button_state_signal.set(State::Error);
                    return;
                }
            };

            let mut created_job = match fyn_api_client.submit_new_job(&new_job_request).await {
                Ok(job) => {
                    leptos::logging::log!("Job created: {}", job.id);
                    job
                }
                Err(e) => {
                    error_signal.set(Some(format!("Failed to submit new job: {}", e)));
                    cl_button_state_signal.set(State::Error);
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
                                error_signal
                                    .set(Some(format!("Failed to upload config file: {}", e)));
                                cl_button_state_signal.set(State::Error);
                            }
                        }
                    }
                    Err(e) => {
                        error_signal.set(Some(format!("Failed to create config file: {}", e)));
                        cl_button_state_signal.set(State::Error);
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
                    error_signal.set(Some(format!(
                        "Error setting job {} ({}) to queue: {:?}",
                        created_job.name, created_job.id, e
                    )));
                    cl_button_state_signal.set(State::Error);
                    return;
                }
            };
            cl_button_text_signal.set("Success".to_string());
            cl_button_state_signal.set(State::Success);
        });
    };

    return view! {
        <div class=format!("w-max {} h-full overflow-y-auto", padding(Size::Md))>
            <Section level={SectionLevel::H2} centre={false} spaced={false} title={"Application Selection".to_string()}>
            // meta data
            {move || {
                let options = applications.list.get().flatten()
                    .unwrap_or(vec![("...".to_string(), "Loading...".to_string())]);
                view! {
                    <FormField
                        label={"Application".to_string()}
                        key={"application".to_string()}
                        input_type=InputType::SelectText {
                            options: options,
                            signal: applications.id
                        }
                    />
                    <FormField
                        label={"Job Name".to_string()}
                        key={"job_name".to_string()}
                        placeholder={"name".to_string()}
                        input_type=InputType::Text {
                            signal: job_name
                        }
                    />
                }
            }}
            </Section>

            // actual input data collection
            {move || {
                match applications.schema.get() {
                    Some(Some(value)) => view! {
                        <Section level={SectionLevel::H2} centre={false} spaced={false} title={"Job Definition".to_string()}>
                                <SchemaForm schema_json=value.to_string() _key=applications.id.get() form_state_out=schema_form_state />
                        </Section>

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
                                <Section level={SectionLevel::H2} centre={false} spaced={false} title={"Job Definition".to_string()}>
                                <FormField
                                    label={"Runner".to_string()}
                                    key={"runner".to_string()}
                                    input_type=InputType::SelectText {
                                        options: runner_options,
                                        signal: runner_id
                                    }
                                />
                                <FormField
                                    label={"Job Priority".to_string()}
                                    key={"job_priority".to_string()}
                                    placeholder={"0".to_string()}
                                    input_type=InputType::Integer { signal: job_priority, min: Some(0), max: Some(100), step: Some(1) }
                                />
                                </Section>
                            }
                        }

                      <Stack align=FlexAlign::Center>
                            <ErrorAlert message={error_message.read_only()} />
                            <Button button_data={
                              let mut button_data = ButtonData::new().on_click(on_submit_click);
                              button_data.state_signal = button_state_signal;
                              button_data.text_signal = button_text_signal;
                              button_data
                          } />
                      </Stack>
                    }.into_any(),
                    Some(None) | None => view!{}.into_any()
               }
             }
            }


        </div>
    };
}
