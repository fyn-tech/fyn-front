use leptos::prelude::*;
use serde::de::Error as ErrorDe;
use serde_json::{Error, Value};

use crate::common::size::*;
use crate::components::atoms::layout::*;
use crate::components::molecules::form_field::*;

fn build_string_form_field(key: &String, object: &Value) -> impl IntoView {
    let title = object["title"].as_str().unwrap_or("none");
    let enum_list = object["enum"].as_array();

    if enum_list.is_none() {
        return view! {<FormField
        label={title.to_string()}
        key={key.to_string()}
        placeholder={"".to_string()}
        input_type=InputType::Text { value: (None)} />};
    } else {
        let options: Vec<(String, String)> = enum_list
            .unwrap()
            .iter()
            .map(|v| {
                let s = v.as_i64().unwrap().to_string();
                (s.clone(), s)
            })
            .collect();
        return view! {<FormField
        label={title.to_string()}
        key={key.to_string()}
        placeholder={"".to_string()}
        input_type=InputType::Select { options: (options), selected: (None) }/> };
    }
}

fn build_integer_form_field(key: &String, object: &Value) -> impl IntoView {
    let title = object["title"].as_str().unwrap_or("none");
    let enum_list = object["enum"].as_array();

    if enum_list.is_none() {
        return view! {<FormField
        label={title.to_string()}
        key={key.to_string()}
        placeholder={"".to_string()}
        input_type=InputType::Integer {
          value: (object["default"].as_i64()),
          min: (object["minimum"].as_i64()),
          max: (object["maximum"].as_i64()),
          step: (None) }/>};
    } else {
        let options: Vec<(String, String)> = enum_list
            .unwrap()
            .iter()
            .map(|v| {
                let s = v.as_i64().unwrap().to_string();
                (s.clone(), s)
            })
            .collect();
        return view! {<FormField
        label={title.to_string()}
        key={key.to_string()}
        placeholder={"".to_string()}
        input_type=InputType::Select { options: (options), selected: (None) }/> };
    }
}

fn build_float_form_field(key: &String, object: &Value) -> impl IntoView {
    let title = object["title"].as_str().unwrap_or("none");

    return view! {<FormField
    label={title.to_string()}
    key={key.to_string()}
    placeholder={"".to_string()}
    input_type=InputType::Float {
      value: (object["default"].as_f64()),
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

    for (field_key, object) in properties.iter() {
        let field_type = object["type"].as_str().unwrap_or("none");

        form_fields.push(match field_type {
            "string" => build_string_form_field(field_key, object).into_any(),
            "integer" => build_integer_form_field(field_key, object).into_any(),
            "number" => build_float_form_field(field_key, object).into_any(),
            _ => continue,
        });
    }

    // For now, return empty vec to get it compiling
    return Ok(form_fields);
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
