use leptos::prelude::*;

use crate::common::size::Size;
use crate::components::atoms::input::*;
use crate::components::atoms::layout::*;
use crate::components::atoms::typography::{FONT_CLR, H4_CLASS};

#[derive(Debug, Clone, PartialEq)]
pub enum InputType {
    Text {
        signal: RwSignal<String>,
    },
    Float {
        signal: RwSignal<Option<f64>>,
        min: Option<f64>,
        max: Option<f64>,
        step: Option<f64>,
    },
    Integer {
        signal: RwSignal<Option<i64>>,
        min: Option<i64>,
        max: Option<i64>,
        step: Option<i64>,
    },
    Email {
        signal: RwSignal<String>,
    },
    Password {
        signal: RwSignal<String>,
    },
    File {
        signal: RwSignal<String>,
    },
    CheckBox {
        signal: RwSignal<bool>,
    },
    SelectText {
        options: Vec<(String, String)>,
        signal: RwSignal<String>,
    },
    SelectInteger {
        options: Vec<(i64, String)>,
        signal: RwSignal<Option<i64>>,
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
        InputType::Text { signal } => view! {
          <Text
            id={id.clone()}
            key={key}
            placeholder={placeholder}
            required={required}
            signal={signal}
          />
        }
        .into_any(),
        InputType::Float {
            signal,
            min,
            max,
            step,
        } => view! {
          <Float
            id={id.clone()}
            key={key}
            placeholder={placeholder}
            required={required}
            signal={signal}
            min={min}
            max={max}
            step={step}
          />
        }
        .into_any(),
        InputType::Integer {
            signal,
            min,
            max,
            step,
        } => view! {
          <Integer
            id={id.clone()}
            key={key}
            placeholder={placeholder}
            required={required}
            signal={signal}
            min={min}
            max={max}
            step={step}
          />
        }
        .into_any(),
        InputType::Email { signal } => view! {
          <Email id={id.clone()} key={key} signal={signal} required={required}/>
        }
        .into_any(),
        InputType::Password { signal } => view! {
          <Password id={id.clone()} key={key} signal={signal} required={required}/>
        }
        .into_any(),
        InputType::File { signal } => view! {
          <File id={id.clone()} key={key} signal={signal} required={required}/>
        }
        .into_any(),
        InputType::CheckBox { signal } => view! {
          <CheckBox id={id.clone()} key={key} signal={signal}/>
        }
        .into_any(),
        InputType::SelectText { options, signal } => view! {
          <SelectText
            id={id.clone()}
            key={key}
            placeholder={placeholder}
            required={required}
            options={options}
            signal={signal}
          />
        }
        .into_any(),
        InputType::SelectInteger { options, signal } => view! {
          <SelectInteger
            id={id.clone()}
            key={key}
            placeholder={placeholder}
            required={required}
            options={options}
            signal={signal}
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
    #[prop(default = true)] horizontal: bool,
    #[prop(optional)] placeholder: Option<String>,
    #[prop(default = false)] required: bool,
) -> impl IntoView {
    let field_id = id.unwrap_or_else(|| format!("field-{}", key));
    let input = build_input(field_id.clone(), key, input_type, required, placeholder);
    let align = if horizontal {
        FlexAlign::Center
    } else {
        FlexAlign::Start
    };
    let spacing = if horizontal { Size::Sm } else { Size::Xs };

    return view! {
      <Stack size={spacing} horizontal={horizontal} align={align} add_class="justify-between".to_string()>
        <label class={format!("{} {}", H4_CLASS, FONT_CLR)} for={field_id.clone()}>{label}</label>
        {input}
      </Stack>
    };
}
