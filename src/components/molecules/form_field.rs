use leptos::prelude::*;
use serde::{Deserialize, Serialize};

use crate::common::size::*;
use crate::components::atoms::button::*;
use crate::components::atoms::input::*;
use crate::components::atoms::layout::*;
use crate::components::atoms::typography::{Size, H3, H4_CLASS, NORMAL_CLASS};

#[derive(Debug, Clone, PartialEq)]
pub enum InputType {
    Text {
        value: Option<String>,
    },
    Float {
        value: Option<f64>,
        min: Option<f64>,
        max: Option<f64>,
        step: Option<f64>,
    },
    Integer {
        value: Option<i64>,
        min: Option<i64>,
        max: Option<i64>,
        step: Option<i64>,
    },
    Email,
    Password,
    File,
    CheckBox,
    Select {
        options: Vec<(String, String)>,
        selected: Option<String>,
    },
}

fn build_input(
    id: String,
    key: String,
    input_type: InputType,
    required: bool,
    placeholder: Option<String>,
) -> impl IntoView {
    return match input_type {
        InputType::Text { value } => view! {
          <Text
            id={id.clone()}
            key={key}
            placeholder={placeholder}
            required={required}
            value={value}
          />
        }
        .into_any(),
        InputType::Float {
            value,
            min,
            max,
            step,
        } => view! {
          <Float
            id={id.clone()}
            key={key}
            placeholder={placeholder}
            required={required}
            value={value}
            min={min}
            max={max}
            step={step}
          />
        }
        .into_any(),
        InputType::Integer {
            value,
            min,
            max,
            step,
        } => view! {
          <Integer
            id={id.clone()}
            key={key}
            placeholder={placeholder}
            required={required}
            value={value}
            min={min}
            max={max}
            step={step}
          />
        }
        .into_any(),
        InputType::Email => view! {
          <Email id={id.clone()} key={key} required={required}/>
        }
        .into_any(),
        InputType::Password => view! {
          <Password id={id.clone()} key={key} required={required}/>
        }
        .into_any(),
        InputType::File => view! {
          <File id={id.clone()} key={key} required={required}/>
        }
        .into_any(),
        InputType::CheckBox => view! {
          <CheckBox id={id.clone()} key={key}/>
        }
        .into_any(),
        InputType::Select { options, selected } => view! {
          <Select
            id={id.clone()}
            key={key}
            placeholder={placeholder}
            required={required}
            options={options}
            selected_value={selected}
          />
        }
        .into_any(),
    };
}

#[component]
pub fn FormField(
    label: String,
    key: String,
    input_type: InputType,

    #[prop(optional)] id: Option<String>,
    #[prop(optional)] placeholder: Option<String>,
    #[prop(default = false)] required: bool,
) -> impl IntoView {
    let field_id = id.unwrap_or_else(|| format!("field-{}", key));

    let class_str = format!("{} {}", H4_CLASS, "");

    let input = build_input(field_id.clone(), key, input_type, required, placeholder);

    return view! {
      <Stack horizontal=true align=FlexAlign::Center add_class="justify-between".to_string()>
        <label class={H4_CLASS} for={field_id.clone()}>{label}</label>
            {input}
      </Stack>
    };
}
