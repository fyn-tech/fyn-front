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
 * filename: button.rs
 * description: Button atomic components with variants, states, and styling
 * ------------------------------------------------------------------------------------------------
 */

use leptos::prelude::*;

use crate::common::size::*;
use crate::components::atoms::layout::*;
use crate::components::atoms::typography::{text_size, FONT_CLR, FONT_STR};

// ------------------------------------------------------------------------------------------------
//  Variant & State
// ------------------------------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
pub enum Variant {
    Primary,
    Secondary,
    Tertiary,
}

impl Variant {
    pub fn base_colour(&self) -> &str {
        match self {
            Variant::Primary => "bg-primary-500 dark:bg-primary-950",
            Variant::Secondary => "bg-primary-300 dark:bg-primary-700",
            Variant::Tertiary => "bg-accent-300 dark:bg-accent-500",
        }
    }

    pub fn hover_colour(&self) -> &str {
        match self {
            Variant::Primary => "hover:bg-primary-300 dark:hover:bg-primary-700",
            Variant::Secondary => "hover:bg-primary-50 dark:hover:bg-primary-500",
            Variant::Tertiary => "hover:bg-accent-50 dark:hover:bg-accent-300",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum State {
    Default,
    Loading,
    Disabled,
    Active,
    Success,
    Error,
}

fn build_class_format(variant: &Variant, state: &State) -> String {
    let mut classes = Vec::new();

    // color
    match state {
        State::Default | State::Active | State::Disabled | State::Loading => {
            classes.push(variant.base_colour())
        }
        State::Success => classes.push("bg-semantic-success"),
        State::Error => classes.push("bg-semantic-error"),
    }

    // accent/effect (non-hover)
    match state {
        State::Default | State::Success | State::Error => classes.push(""),
        State::Active => classes.push("ring-2 ring-primary-950 dark:ring-primary-300"),
        State::Disabled => classes.push("opacity-50 cursor-not-allowed"),
        State::Loading => classes.push("opacity-75"),
    }

    // hover
    match state {
        State::Default => classes.push(variant.hover_colour()),
        State::Active | State::Disabled | State::Loading | State::Success | State::Error => {
            classes.push("")
        }
    }

    classes.join(" ")
}

// ------------------------------------------------------------------------------------------------
//  Type
// ------------------------------------------------------------------------------------------------

// TODO TYPES: will add as we go along.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Standard, // Default clickable button
    Toggle,   // On/off state button
    Radio,    // Single selection from group
    Checkbox, // Multiple selection
}

// ------------------------------------------------------------------------------------------------
//  Components
// ------------------------------------------------------------------------------------------------

#[component]
pub fn Button(
    #[prop(default = Variant::Primary)] variant: Variant,
    #[prop(default = State::Default)] state: State,
    #[prop(default = Size::Md)] size: Size,
    #[prop(default = "Click me".to_string())] text: String,
    #[prop(optional)] on_click: Option<Box<dyn Fn() + 'static>>,
) -> impl IntoView {
    let padding = padding(size);

    let text_format = format!("{} {} {}", FONT_STR, text_size(size), FONT_CLR);

    let button_classes = format!(
        "{} {} {} {}",
        build_class_format(&variant, &state),
        ROUND_BORDER,
        padding,
        text_format
    );

    return view! {
        <button
            id=format!("btn-{:?}", size)
            class={button_classes}
            on:click=move |_| {
                if let Some(ref action) = on_click {
                    action();
                }
            }
        >
            {text}
        </button>
    };
}

#[component]
pub fn GroupButton(
    #[prop(default = State::Default)] state: State,
    #[prop(default = Size::Md)] size: Size,
    #[prop(default = "Click me".to_string())] text: String,
    #[prop(optional)] on_click: Option<Box<dyn Fn() + 'static>>,
) -> impl IntoView {
    let padding = padding(size);

    let text_format = format!("{} {} {}", FONT_STR, text_size(size), FONT_CLR);

    let button_classes = format!(
        "{} {} {}",
        build_class_format(&Variant::Primary, &state),
        padding,
        text_format
    );

    return view! {
        <button
            id=format!("btn-{:?}", size)
            class={button_classes}
            on:click=move |_| {
                if let Some(ref action) = on_click {
                    action();
                }
            }
        >
            {text}
        </button>
    };
}
