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
use crate::infrastructure::fyn_api_client::FynApiClient;

#[component]
pub fn JobConfigForm() -> impl IntoView {
    let fyn_api_client = use_context::<FynApiClient>().expect("FynApiClient should be provided");

    let application = RwSignal::new(String::new());

    // Use LocalResource instead of spawn_local + RwSignal
    let application_list = LocalResource::new({
        let api_client = fyn_api_client.clone();
        move || {
            let api_client = api_client.clone(); // Clone inside the closure
            async move { api_client.get_applications().await }
        }
    });

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
