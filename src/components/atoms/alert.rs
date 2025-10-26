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
 * filename: alert.rs
 * description: Alert/message atomic components for user feedback
 * ------------------------------------------------------------------------------------------------
 */

use leptos::prelude::*;

use crate::common::size::*;
use crate::components::atoms::layout::*;
use crate::components::atoms::typography::{text_size, FONT_STR};

// ------------------------------------------------------------------------------------------------
//  Alert Type
// ------------------------------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(dead_code)]
pub enum AlertType {
    Error,
    Warning,
    Success,
    Info,
}

impl AlertType {
    pub fn background_color(&self) -> &str {
        match self {
            AlertType::Error => "bg-red-100 dark:bg-red-900/20",
            AlertType::Warning => "bg-yellow-100 dark:bg-yellow-900/20",
            AlertType::Success => "bg-green-100 dark:bg-green-900/20",
            AlertType::Info => "bg-blue-100 dark:bg-blue-900/20",
        }
    }

    pub fn border_color(&self) -> &str {
        match self {
            AlertType::Error => "border-red-400 dark:border-red-500",
            AlertType::Warning => "border-yellow-400 dark:border-yellow-500",
            AlertType::Success => "border-green-400 dark:border-green-500",
            AlertType::Info => "border-blue-400 dark:border-blue-500",
        }
    }

    pub fn text_color(&self) -> &str {
        match self {
            AlertType::Error => "text-red-500 dark:text-red-500",
            AlertType::Warning => "text-yellow-500 dark:text-yellow-500",
            AlertType::Success => "text-green-500 dark:text-green-500",
            AlertType::Info => "text-blue-500 dark:text-blue-500",
        }
    }
}

// ------------------------------------------------------------------------------------------------
//  Components
// ------------------------------------------------------------------------------------------------

#[component]
pub fn Alert(
    message: ReadSignal<Option<String>>,
    #[prop(default = AlertType::Info)] alert_type: AlertType,
    #[prop(default = Size::Md)] size: Size,
) -> impl IntoView {
    let padding_class = padding(size);
    let text_size_class = text_size(size);

    let alert_classes = format!(
        "{} {} {} {} {} {} {} {} {}",
        alert_type.background_color(),
        "border",
        alert_type.border_color(),
        alert_type.text_color(),
        padding_class,
        "rounded",
        "mb-4",
        FONT_STR,
        text_size_class
    );

    return move || match message.get() {
        Some(msg) => view! { <div class={alert_classes.clone()}>{msg}</div> }.into_any(),
        None => view! {}.into_any(),
    };
}

#[component]
pub fn ErrorAlert(
    message: ReadSignal<Option<String>>,
    #[prop(default = Size::Md)] size: Size,
) -> impl IntoView {
    return view! {
        <Alert alert_type=AlertType::Error size={size} message={message} />
    };
}

#[component]
pub fn WarningAlert(
    message: ReadSignal<Option<String>>,
    #[prop(default = Size::Md)] size: Size,
) -> impl IntoView {
    return view! {
        <Alert alert_type=AlertType::Warning size={size} message={message} />
    };
}

#[component]
pub fn SuccessAlert(
    message: ReadSignal<Option<String>>,
    #[prop(default = Size::Md)] size: Size,
) -> impl IntoView {
    return view! {
        <Alert alert_type=AlertType::Success size={size} message={message} />
    };
}

#[component]
pub fn InfoAlert(
    message: ReadSignal<Option<String>>,
    #[prop(default = Size::Md)] size: Size,
) -> impl IntoView {
    return view! {
        <Alert alert_type=AlertType::Info size={size} message={message} />
    };
}
