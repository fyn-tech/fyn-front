use leptos::prelude::*;

use crate::common::size::*;
use crate::components::atoms::button::*;
use crate::components::atoms::layout::*;
use crate::components::molecules::schema_form::SchemaForm;
use crate::components::molecules::section::*;

#[component]
pub fn JobConfigForm() -> impl IntoView {
    return view! {
      <div class=format!("w-max {} h-full overflow-y-auto", padding(Size::Md))>
          <Section level={SectionLevel::H2} centre={false} spaced={false} title={"Application Name".to_string()}>
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
