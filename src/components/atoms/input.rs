use std::default;

use leptos::prelude::*;
use serde::de::value;

use crate::common::size::Size;
use crate::components::atoms::button::*;
use crate::components::atoms::layout::*;
use crate::components::atoms::typography::{FONT_CLR, FONT_STR};

// -------------------------------------------------------------------------------------------------
//  Common Input
// -------------------------------------------------------------------------------------------------

fn input_field_string(align: Align) -> String {
    return format!(
        "{} {} {} {} {} {} {} {} {}",
        "bg-surface-50 dark:bg-surface-950",
        "w-64",
        standard_border(Some(BorderColor::Surface)),
        ROUND_BORDER,
        padding(Size::Sm),
        "invalid:border-red-500 invalid:ring-red-500",
        align,
        FONT_STR,
        FONT_CLR
    );
}

// -------------------------------------------------------------------------------------------------
//  Components
// -------------------------------------------------------------------------------------------------

#[component]
pub fn Text(
    id: String,
    key: String,
    #[prop(default = None)] value: Option<String>,
    #[prop(default = None)] placeholder: Option<String>,
    #[prop(default = false)] required: bool,
) -> impl IntoView {
    let class_str = input_field_string(Align::Left);

    return view! {
        <input
            class={class_str}
            type="text" id={id}
            name={key}
            value={value.unwrap_or("".to_string())}
            placeholder={placeholder.unwrap_or("text".to_string())}
            required={required}
        />
    };
}

#[component]
pub fn Float(
    id: String,
    key: String,
    #[prop(default = None)] placeholder: Option<String>,
    #[prop(default = false)] required: bool,
    #[prop(default = None)] value: Option<f64>,
    #[prop(default = None)] min: Option<f64>,
    #[prop(default = None)] max: Option<f64>,
    #[prop(default = None)] step: Option<f64>,
) -> impl IntoView {
    let class_str = input_field_string(Align::Left);

    return view! {
        <input
            class={class_str}
            type="number"
            id={id}
            name={key}
            value={value}
            placeholder={placeholder.unwrap_or("enter value".to_string())}
            required={required}
            min={min}
            max={max}
            step={step.map_or("any".to_string(), |s| s.to_string())}
        />
    };
}

#[component]
pub fn Integer(
    id: String,
    key: String,
    #[prop(default = None)] placeholder: Option<String>,
    #[prop(default = false)] required: bool,
    #[prop(default = None)] value: Option<i64>,
    #[prop(default = None)] min: Option<i64>,
    #[prop(default = None)] max: Option<i64>,
    #[prop(default = None)] step: Option<i64>,
) -> impl IntoView {
    let class_str = input_field_string(Align::Left);

    return view! {
        <input
            class={class_str}
            type="number"
            id={id}
            name={key}
            value={value}
            placeholder={placeholder.unwrap_or("enter value".to_string())}
            required={required}
            min={min}
            max={max}
            step={step.map_or("1".to_string(), |s| s.to_string())}
        />
    };
}

#[component]
pub fn Email(
    id: String,
    key: String,
    #[prop(default = "e-mail".to_string())] placeholder: String,
    #[prop(default = false)] required: bool,
) -> impl IntoView {
    let class_str = input_field_string(Align::Left);

    return view! {
        <input
            class={class_str}
            type="email"
            id={id}
            name={key}
            placeholder={placeholder}
            required={required}
        />
    };
}

#[component]
pub fn Password(
    id: String,
    key: String,
    #[prop(default = "password".to_string())] placeholder: String,
    #[prop(default = false)] required: bool,
) -> impl IntoView {
    let class_str = input_field_string(Align::Left);

    return view! {
        <input
            class={class_str}
            type="password"
            id={id}
            name={key}
            placeholder={placeholder}
            required={required}
        />
    };
}

/*TODO: fixed abstract the styling away. */
#[component]
pub fn File(id: String, key: String, #[prop(default = false)] required: bool) -> impl IntoView {
    let class_str = format!(
        "{} 
        file:bg-primary-500 dark:file:bg-primary-950 
        file:hover:bg-primary-300 dark:file:hover:bg-primary-700
        file:{} 
        file:border-0 
        file:px-{} file:py-{}
        file:{}
        file:text-content-primary dark:file:text-content-primary-dark",
        input_field_string(Align::Left),
        ROUND_BORDER,
        spacing(Size::Sm),
        spacing(Size::Xs),
        FONT_STR,
    );

    return view! {
        <input
            class={class_str}
            type="file"
            id={id}
            name={key}
            required={required}
        />
    };
}

#[component]
pub fn CheckBox(id: String, key: String) -> impl IntoView {
    let class_str = format!(
        "w-5 h-5 {} {} {} {} {}",
        "bg-surface-50 dark:bg-surface-950",
        standard_border(Some(BorderColor::Surface)),
        ROUND_BORDER,
        "accent-primary-500",
        "focus:ring-2 focus:ring-primary-300 focus:ring-opacity-50"
    );

    return view! {
        <input
            class={class_str}
            type="checkbox"
            id={id}
            name={key}

        />
    };
}

#[component]
pub fn Select(
    id: String,
    key: String,
    options: Vec<(String, String)>, // (value, display_text) pairs
    #[prop(default = None)] placeholder: Option<String>,
    #[prop(default = false)] required: bool,
    #[prop(default = None)] selected_value: Option<String>,
) -> impl IntoView {
    let class_str = input_field_string(Align::Left);

    return view! {
        <select
            class={class_str}
            id={id}
            name={key}
            required={required}
        >

            {if placeholder.is_none() {
                Some(view! {
                    <option value="" disabled={true} selected={selected_value.is_none()}>
                        {placeholder.unwrap_or("select option".to_string())}
                    </option>
                })
            } else {
                None
            }}

            {options.into_iter().map(|(value, text)| {
                let is_selected = selected_value.as_ref() == Some(&value);
                view! {
                    <option value={value} selected={is_selected}>
                        {text}
                    </option>
                }
            }).collect_view()}
        </select>
    };
}
