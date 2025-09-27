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
use leptos::prelude::*;

use crate::common::size::*;
use crate::components::atoms::button::*;
use crate::components::atoms::layout::*;
use crate::components::molecules::form_field::*;
use crate::components::molecules::schema_form::SchemaForm;
use crate::components::molecules::section::*;
use crate::domain::user_context::UserContext;
use crate::infrastructure::fyn_api_client::FynApiClient;

fn get_application_list() -> LocalResource<Option<Vec<(String, String)>>> {
    LocalResource::new({
        move || async move {
            let fyn_api_client =
                use_context::<FynApiClient>().expect("FynApiClient should be provided");
            let mut user_context =
                use_context::<RwSignal<Option<UserContext>>>().expect("user should be provided");
            if user_context.get().is_some() && user_context.get().unwrap().apps.len() == 0 {
                let app_info: Option<Vec<crate::domain::application_info::AppInfo>> =
                    fyn_api_client.get_applications().await;
                let mut user_cont = user_context.get();
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
                    .map(|app| (app.id.to_string(), app.name.clone()))
                    .collect(),
            )
        }
    })
}

#[component]
pub fn JobConfigForm() -> impl IntoView {
    let application = RwSignal::new(String::new());
    let application_list = get_application_list();

    return view! {
        <div class=format!("w-max {} h-full overflow-y-auto", padding(Size::Md))>
            <Section level={SectionLevel::H2} centre={false} spaced={false} title={"Application Name".to_string()}>
            // meta data
            {move || {
                let options = application_list.get()
                    .flatten()
                    .unwrap_or(vec![("...".to_string(), "Loading...".to_string())]);
                view! {
                    <FormField
                        label={"Application".to_string()}
                        key={"application".to_string()}
                        input_type=InputType::SelectText {
                            options: options,
                            signal: application
                        }
                    />
                }
            }}
            // actual input data collection
            <SchemaForm schema_json={PLACEHOLDER_SCHEMA.to_string()}/>
            </Section>
            <Stack align=FlexAlign::Center>
                <Button text="Submit Job".to_string()/>
            </Stack>
        </div>
    };
}

pub const PLACEHOLDER_SCHEMA: &str = r#"
{
  "type": "object",
  "properties": {
    "analysis_name": {
      "type": "string",
      "title": "Analysis Name",
      "description": "Name for this analysis"
    },
    "dim": {
      "type": "integer",
      "title": "Dimensions",
      "enum": [2, 3] 
    },
    "cells_x": {
      "type": "integer",
      "title": "Cells X",
      "minimum": 1,
      "maximum": 1000
    },
    "cells_y": {
      "type": "integer",
      "title": "Cells Y",
      "minimum": 1,
      "maximum": 1000
    },
    "time_step_size": {
      "type": "number",
      "title": "Time Step Size",
      "minimum": 0.0001,
      "maximum": 1.0
    },
    "time_begin": {
      "type": "number",
      "title": "Start Time",
      "minimum": 0.0
    },
    "time_end": {
      "type": "number",
      "title": "End Time",
      "minimum": 0.1
    },
    "density": {
      "type": "number",
      "title": "Density",
      "minimum": 0.1
    },
    "viscosity": {
      "type": "number",
      "title": "Viscosity",
      "minimum": 0.001
    }
  },
  "required": ["analysis_name", "time_step_size", "time_end", "density", "viscosity"]
}
"#;
