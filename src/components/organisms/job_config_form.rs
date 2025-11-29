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
use serde_json::{json, Value};
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

#[derive(Clone, Copy, Default)]
struct JobFormData {
    // Job Configuration
    pub app_id: RwSignal<Option<Uuid>>,
    pub job_name: RwSignal<String>,
    pub executable_name: RwSignal<String>,
    pub command_line_args: RwSignal<String>,

    // Job Inputs
    pub schema_form: RwSignal<Option<SchemaFormState>>,

    // Compute Allocation
    pub runner_id: RwSignal<Option<Uuid>>,
    pub job_priority: RwSignal<Option<i64>>,
}

impl JobFormData {
    fn new() -> Self {
        Self {
            app_id: RwSignal::new(None),
            job_name: RwSignal::new("".to_string()),
            executable_name: RwSignal::new("".to_string()),
            command_line_args: RwSignal::new("".to_string()),
            schema_form: RwSignal::new(None),
            runner_id: RwSignal::new(None),
            job_priority: RwSignal::new(None),
        }
    }

    // fn validate(&self) -> Result<ValidatedJobSubmission, Vec<ValidationError>> {}
}

struct JobSubmissionService {
    api_client: FynApiClient,
}

impl JobSubmissionService {
    async fn submit_job(&self, form_data: &JobFormData) -> Result<JobInfo, String> {}
}

async fn submit_job(
    fyn_api_client: &FynApiClient,
    form_data: &JobFormData,
) -> Result<JobInfo, String> {
    // Helper for UUID parsing
    let parse_uuid = |s: &str, field: &str| -> Result<Uuid, String> {
        Uuid::from_str(s).map_err(|e| format!("Invalid {}: {:?}", field, e))
    };

    let cli_conv = form_data
        .command_line_args
        .get()
        .split_whitespace()
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    let new_job_request = JobInfo::new()
        .name(form_data.job_name.get())
        .application_id(form_data.app_id.get().unwrap_or_default())
        .runner_id(form_data.runner_id.get().unwrap_or_default())
        .priority(form_data.job_priority.get().unwrap_or(0i64))
        .executable(form_data.executable_name.get())
        .command_line_args(&json!(cli_conv))
        .build()
        .map_err(|e| format!("Failed to create job: {:?}", e))?;

    let mut created_job = fyn_api_client
        .submit_new_job(&new_job_request)
        .await
        .map_err(|e| format!("Failed to submit new job: {}", e))?;

    leptos::logging::log!("Job created: {}", created_job.id);

    // Upload config file if we have form data
    if let Some(form_state) = form_data.schema_form.get() {
        let config_json = form_state.to_json();
        leptos::logging::log!("Creating config file from form data");

        let config_file = FynApiClient::create_json_file(&config_json, "config_file.json")
            .map_err(|e| format!("Failed to create config file: {}", e))?;

        leptos::logging::log!("Uploading config file for job {}", created_job.id);

        fyn_api_client
            .upload_job_resource_file(
                created_job.id,
                config_file,
                "CFG", // CONFIG resource type
                Some("Application configuration file"),
            )
            .await
            .map_err(|e| format!("Failed to upload config file: {}", e))?;

        leptos::logging::log!("Config file uploaded successfully");
    } else {
        leptos::logging::log!("No form data to upload");
    }

    // Place into queued state to trigger runner pick up
    created_job.status = JobStatus::Queued;
    let job_info = fyn_api_client.patch_job(&created_job).await.map_err(|e| {
        format!(
            "Error setting job {} ({}) to queue: {:?}",
            created_job.name, created_job.id, e
        )
    })?;

    leptos::logging::log!("Job {} ({}) set to queued", job_info.name, job_info.id);

    Ok(job_info)
}

// -------------------------------------------------------------------------------------------------
// Component
// -------------------------------------------------------------------------------------------------

#[component]
pub fn JobConfigForm(
    runner_list: Option<HashMap<Uuid, RunnerInfo>>,
    applications: ApplicationSchemaConfig,
) -> impl IntoView {
    let job_data = JobFormData::new();

    let error_message = RwSignal::new(None::<String>);

    // Button state signal
    let button_state_signal = RwSignal::new(State::Default);
    let button_text_signal = RwSignal::new("Submit".to_string());

    let on_submit_click = move || {
        let fyn_api_client =
            use_context::<FynApiClient>().expect("FynApiClient should be provided");
        let cl_button_state_signal = button_state_signal.clone();
        let cl_button_text_signal = button_text_signal.clone();

        spawn_local(async move {
            cl_button_state_signal.set(State::Loading);
            error_message.set(None);

            match submit_job(&fyn_api_client, &job_data).await {
                Ok(_job_info) => {
                    cl_button_text_signal.set("Success".to_string());
                    cl_button_state_signal.set(State::Success);
                }
                Err(e) => {
                    error_message.set(Some(e));
                    cl_button_state_signal.set(State::Error);
                }
            }
        });
    };

    job_data.command_line_args.set("--input yml".to_string());

    return view! {
        <div class=format!("w-max {} h-full overflow-y-auto", padding(Size::Md))>
            <Section level={SectionLevel::H2} centre={false} spaced={false} title={"Application Configuration".to_string()}>
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
                            signal: job_data.job_name
                        }
                    />
                    <FormField
                        label={"CL Args".to_string()}
                        key={"cl_args".to_string()}
                        placeholder={"args".to_string()}
                        input_type=InputType::Text {
                            signal: job_data.command_line_args
                        }
                    />
                }
            }}
            </Section>

            // actual input data collection
            {move || {
                match applications.schema.get() {
                    Some(Some(value)) => view! {
                        <Section level={SectionLevel::H2} centre={false} spaced={false} title={"Application Input".to_string()}>
                                <SchemaForm schema_json=value.to_string() form_state_out=job_data.schema_form />
                        </Section>
                        <JobDefinition runner_list=runner_list.clone() runner_id=job_data.runner_id job_priority=job_data.job_priority />

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

#[component]
fn JobDefinition(
    runner_list: Option<HashMap<Uuid, RunnerInfo>>,
    runner_id: RwSignal<Option<Uuid>>,
    job_priority: RwSignal<Option<i64>>,
) -> impl IntoView {
    let runner_options = match &runner_list.clone() {
        Some(runners) => runners
            .iter()
            .map(|(id, runner)| (id.clone(), runner.name.clone()))
            .collect::<Vec<(Uuid, String)>>(),
        None => vec![(Uuid::nil(), "Loading runners...".to_string())],
    };

    view! {
        <Section level={SectionLevel::H2} centre={false} spaced={false} title={"Compute Allocation".to_string()}>
            <FormField
                label={"Runner".to_string()}
                key={"runner".to_string()}
                input_type=InputType::SelectUuid {
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
