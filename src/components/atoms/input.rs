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
 * filename: input.rs
 * description: Input atomic components
 * ------------------------------------------------------------------------------------------------
 */

use leptos::prelude::*;
use uuid::Uuid;

use crate::common::size::Size;
use crate::components::atoms::layout::*;
use crate::components::atoms::typography::{FONT_CLR, FONT_STR};

// ------------------------------------------------------------------------------------------------
//  Common Input
// ------------------------------------------------------------------------------------------------

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

// ------------------------------------------------------------------------------------------------
//  Components
// ------------------------------------------------------------------------------------------------

#[component]
pub fn Text(
    id: String,
    key: String,
    signal: RwSignal<String>,
    #[prop(default = None)] placeholder: Option<String>,
    #[prop(default = false)] required: bool,
) -> impl IntoView {
    let class_str = input_field_string(Align::Left);

    return view! {
        <input
            class={class_str}
            type="text"
            id={id}
            name={key}
            placeholder={placeholder.unwrap_or("text".to_string())}
            required={required}
            prop:value=move || signal.get()
            on:input=move |ev| {
                signal.set(event_target_value(&ev));
            }
        />
    };
}

#[component]
pub fn Float(
    id: String,
    key: String,
    signal: RwSignal<Option<f64>>,
    #[prop(default = None)] placeholder: Option<String>,
    #[prop(default = false)] required: bool,
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
            placeholder={placeholder.unwrap_or("enter value".to_string())}
            required={required}
            min={min}
            max={max}
            step={step.map_or("any".to_string(), |s| s.to_string())}
            prop:value=move || signal.get().map(|v| v.to_string()).unwrap_or_default()
            on:input=move |ev| {
                let input_str = event_target_value(&ev);
                if input_str.is_empty() {
                    signal.set(None);
                } else {
                    signal.set(input_str.parse::<f64>().ok());
                }
            }
        />
    };
}

#[component]
pub fn Integer(
    id: String,
    key: String,
    signal: RwSignal<Option<i64>>,
    #[prop(default = None)] placeholder: Option<String>,
    #[prop(default = false)] required: bool,
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
            placeholder={placeholder.unwrap_or("enter value".to_string())}
            required={required}
            min={min}
            max={max}
            step={step.map_or("1".to_string(), |s| s.to_string())}
            prop:value=move || signal.get().map(|v| v.to_string()).unwrap_or_default()
            on:input=move |ev| {
                let input_str = event_target_value(&ev);
                if input_str.is_empty() {
                    signal.set(None);
                } else {
                    signal.set(input_str.parse::<i64>().ok());
                }
            }
        />
    };
}

#[component]
pub fn Email(
    id: String,
    key: String,
    signal: RwSignal<String>,
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
            prop:value=move || signal.get()
            on:input=move |ev| {
                signal.set(event_target_value(&ev));
            }
        />
    };
}

#[component]
pub fn Password(
    id: String,
    key: String,
    signal: RwSignal<String>,
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
            prop:value=move || signal.get()
            on:input=move |ev| {
                signal.set(event_target_value(&ev));
            }
        />
    };
}

/*TODO: fixed abstract the styling away. */
#[component]
pub fn File(
    id: String,
    key: String,
    signal: RwSignal<String>,
    #[prop(default = false)] required: bool,
) -> impl IntoView {
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
            prop:value=move || signal.get()
            on:input=move |ev| {
                signal.set(event_target_value(&ev));
            }
        />
    };
}

#[component]
pub fn CheckBox(id: String, key: String, signal: RwSignal<bool>) -> impl IntoView {
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
            prop:checked=move || signal.get()
            on:change=move |ev| {
                let checked = event_target_checked(&ev);
                signal.set(checked);
            }
        />
    };
}

#[component]
pub fn SelectText(
    id: String,
    key: String,
    options: Vec<(String, String)>, // (value, display_text) pairs
    signal: RwSignal<String>,
    #[prop(default = None)] placeholder: Option<String>,
    #[prop(default = false)] required: bool,
) -> impl IntoView {
    let class_str = input_field_string(Align::Left);

    return view! {
        <select
            class={class_str}
            id={id}
            name={key}
            required={required}
            prop:value=move || signal.get()
            on:change=move |ev| {
                signal.set(event_target_value(&ev));
            }
        >
            // Placeholder option (only if provided)
            {if let Some(placeholder_text) = placeholder {
                Some(view! {
                    <option value="" disabled=true selected={signal.get().is_empty()}>
                        {placeholder_text}
                    </option>
                })
            } else {
                None
            }}

            // Regular options
            {options.into_iter().map(|(value, text)| {
                view! {
                    <option
                        value={value.clone()}
                        selected={move || signal.get() == value}
                    >
                        {text}
                    </option>
                }
            }).collect_view()}
        </select>
    };
}

#[component]
pub fn SelectInteger(
    id: String,
    key: String,
    options: Vec<(i64, String)>,
    signal: RwSignal<Option<i64>>,
    #[prop(default = None)] placeholder: Option<String>,
    #[prop(default = false)] required: bool,
) -> impl IntoView {
    let class_str = input_field_string(Align::Left);

    return view! {
        <select
            class={class_str}
            id={id}
            name={key}
            required={required}
            prop:value=move || {  // Convert Option<i64> to String for HTML
                signal.get()
                    .map(|v| v.to_string())
                    .unwrap_or_default()
            }
            on:change=move |ev| {  // Parse String back to Option<i64>
                let value_str = event_target_value(&ev);
                if value_str.is_empty() {
                    signal.set(None);
                } else {
                    signal.set(value_str.parse::<i64>().ok());
                }
            }
        >
            // Placeholder option (only if provided)
            {if let Some(placeholder_text) = placeholder {
                Some(view! {
                    <option value="" disabled=true selected={signal.get().is_none()}>  // Fixed: is_none()
                        {placeholder_text}
                    </option>
                })
            } else {
                None
            }}

            // Regular options
            {options.into_iter().map(|(value, text)| {
                view! {
                    <option
                        value={value.to_string()}  // Convert i64 to String for HTML
                        selected={signal.get() == Some(value)}  // Fixed: compare with Some(value)
                    >
                        {text}
                    </option>
                }
            }).collect_view()}
        </select>
    };
}

#[component]
pub fn SelectUuid(
    id: String,
    key: String,
    options: Vec<(Uuid, String)>,
    signal: RwSignal<Option<Uuid>>,
    #[prop(default = None)] placeholder: Option<String>,
    #[prop(default = false)] required: bool,
) -> impl IntoView {
    let class_str = input_field_string(Align::Left);

    return view! {
        <select
            class={class_str}
            id={id}
            name={key}
            required={required}
            prop:value=move || {
                signal.get()
                    .map(|v| v.to_string())
                    .unwrap_or_default()
            }
            on:change=move |ev| {
                let value_str = event_target_value(&ev);
                if value_str.is_empty() {
                    signal.set(None);
                } else {
                    signal.set(value_str.parse::<Uuid>().ok());
                }
            }
        >
            // Placeholder option (only if provided)
            {if let Some(placeholder_text) = placeholder {
                Some(view! {
                    <option value="" disabled=true selected={signal.get().is_none()}>
                        {placeholder_text}
                    </option>
                })
            } else {
                None
            }}

            // Regular options
            {options.into_iter().map(|(value, text)| {
                view! {
                    <option
                        value={value.to_string()}
                        selected={signal.get() == Some(value)}
                    >
                        {text}
                    </option>
                }
            }).collect_view()}
        </select>
    };
}
