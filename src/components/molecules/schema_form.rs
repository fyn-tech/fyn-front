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
 * filename: schema_form.rs
 * description: Schema-driven form molecule component
 * ------------------------------------------------------------------------------------------------
 */


use leptos::prelude::*;
use serde::de::Error as ErrorDe;
use serde_json::{Error, Value};
use std::collections::HashMap;
use std::default;

use crate::common::size::*;
use crate::components::atoms::layout::*;
use crate::components::molecules::form_field::*;

fn build_string_form_field(
    key: &String,
    object: &Value,
    signal: RwSignal<String>,
) -> impl IntoView {
    let title = object["title"].as_str().unwrap_or("none");
    let enum_list = object["enum"].as_array();

    if enum_list.is_none() {
        return view! {<FormField
        label={title.to_string()}
        key={key.to_string()}
        placeholder={"".to_string()}
        input_type=InputType::Text { signal: signal} />};
    } else {
        let options: Vec<(String, String)> = enum_list
            .unwrap()
            .iter()
            .map(|v| {
                if let Some(string_val) = v.as_str() {
                    (string_val.to_string(), string_val.to_string())
                } else if let Some(int_val) = v.as_i64() {
                    let s = int_val.to_string();
                    (s.clone(), s)
                } else {
                    ("unknown".to_string(), "unknown".to_string())
                }
            })
            .collect();

        // set a default if none was provided
        if signal.get() == "".to_string() {
            signal.set(options[0].0.clone());
        }

        return view! {<FormField
        label={title.to_string()}
        key={key.to_string()}
        placeholder={"".to_string()}
        input_type=InputType::SelectText { options: (options), signal: signal }/> };
    }
}

fn build_integer_form_field(
    key: &String,
    object: &Value,
    signal: RwSignal<Option<i64>>,
) -> impl IntoView {
    let title = object["title"].as_str().unwrap_or("none");
    let enum_list = object["enum"].as_array();

    signal.set(object["default"].as_i64());

    if enum_list.is_none() {
        return view! {<FormField
        label={title.to_string()}
        key={key.to_string()}
        placeholder={"".to_string()}
        input_type=InputType::Integer {
          signal: signal,
          min: (object["minimum"].as_i64()),
          max: (object["maximum"].as_i64()),
          step: (None) }/>};
    } else {
        let options: Vec<(i64, String)> = enum_list
            .unwrap()
            .iter()
            .map(|v| {
                let int_val = v.as_i64().unwrap();
                (int_val, int_val.to_string())
            })
            .collect();

        // set a default if none was provided
        if signal.get().is_none() {
            signal.set(Some(options[0].0.clone()));
        }

        return view! {<FormField
        label={title.to_string()}
        key={key.to_string()}
        placeholder={"".to_string()}
        input_type=InputType::SelectInteger { options: (options), signal: signal }/> };
    }
}

fn build_float_form_field(
    key: &String,
    object: &Value,
    signal: RwSignal<Option<f64>>,
) -> impl IntoView {
    let title = object["title"].as_str().unwrap_or("none");
    signal.set(object["default"].as_f64());

    return view! {<FormField
    label={title.to_string()}
    key={key.to_string()}
    placeholder={"".to_string()}
    input_type=InputType::Float {
      signal: signal,
      min: (object["minimum"].as_f64()),
      max: (object["maximum"].as_f64()),
      step: (None) }/>};
}

fn schema_to_form_fields(schema_json: &str) -> Result<Vec<AnyView>, Error> {
    let schema: Value = serde_json::from_str(schema_json)?;
    let properties = schema["properties"]
        .as_object()
        .ok_or_else(move || ErrorDe::custom("Missing 'properties' field in schema"))?;

    let mut form_fields = Vec::new();
    let mut form_state = SchemaFormState::default();

    for (field_key, object) in properties.iter() {
        let field_type = object["type"].as_str().unwrap_or("none");

        form_fields.push(match field_type {
            "string" => {
                form_state
                    .text_signals
                    .insert(field_key.clone(), RwSignal::new(String::new()));
                build_string_form_field(field_key, object, form_state.text_signals[field_key])
                    .into_any()
            }
            "integer" => {
                form_state
                    .int_signals
                    .insert(field_key.clone(), RwSignal::new(None));
                build_integer_form_field(field_key, object, form_state.int_signals[field_key])
                    .into_any()
            }
            "number" => {
                form_state
                    .float_signals
                    .insert(field_key.clone(), RwSignal::new(None));
                build_float_form_field(field_key, object, form_state.float_signals[field_key])
                    .into_any()
            }
            _ => continue,
        });
    }

    // For now, return empty vec to get it compiling
    return Ok(form_fields);
}

#[derive(Clone, Default)]
pub struct SchemaFormState {
    text_signals: HashMap<String, RwSignal<String>>,
    float_signals: HashMap<String, RwSignal<Option<f64>>>,
    int_signals: HashMap<String, RwSignal<Option<i64>>>,
}

#[component]
pub fn SchemaForm(schema_json: String) -> impl IntoView {
    let field_views = match schema_to_form_fields(&schema_json) {
        Ok(fields) => fields,
        Err(e) => {
            log::error!("Schema parsing error: {:?}", e);
            vec![] // Return empty on error
        }
    };
    return view! {
        <Grid size={Size::Md} cols=1>
            {field_views.into_iter().collect::<Vec<_>>()}
        </Grid>
    };
}
