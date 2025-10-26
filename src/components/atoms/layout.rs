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
 * filename: layout.rs
 * description: Layout atomic components
 * ------------------------------------------------------------------------------------------------
 */

use leptos::prelude::*;

use crate::common::size::*;

// ------------------------------------------------------------------------------------------------
//  Alignment
// ------------------------------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Align {
    Left,
    Center,
    Right,
    Justify,
}

impl std::fmt::Display for Align {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let class = match self {
            Align::Left => "text-left",
            Align::Center => "text-center",
            Align::Right => "text-right",
            Align::Justify => "text-justify",
        };
        return write!(f, "{}", class);
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(dead_code)]
pub enum FlexAlign {
    Start,
    Center,
    End,
    Stretch,
    Baseline,
}

impl std::fmt::Display for FlexAlign {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let class = match self {
            FlexAlign::Start => "items-start",
            FlexAlign::Center => "items-center",
            FlexAlign::End => "items-end",
            FlexAlign::Stretch => "items-stretch",
            FlexAlign::Baseline => "items-baseline",
        };
        write!(f, "{}", class)
    }
}

// ------------------------------------------------------------------------------------------------
//  Spacing and Padding
// ------------------------------------------------------------------------------------------------

pub fn spacing(size: Size) -> &'static str {
    return match size {
        Size::None => "0",
        Size::Xs => "1",
        Size::Sm => "2",
        Size::Md => "3",
        Size::Lg => "4",
        Size::Xl => "5",
        Size::Xl2 => "6",
    };
}

pub fn padding(size: Size) -> String {
    return match size {
        Size::None => format!("px-{} py-{}", spacing(Size::None), spacing(Size::None)),
        Size::Xs => format!("px-{} py-{}", spacing(Size::Xs), spacing(Size::Xs)),
        Size::Sm => format!("px-{} py-{}", spacing(Size::Sm), spacing(Size::Xs)),
        Size::Md => format!("px-{} py-{}", spacing(Size::Md), spacing(Size::Sm)),
        Size::Lg => format!("px-{} py-{}", spacing(Size::Lg), spacing(Size::Md)),
        Size::Xl => format!("px-{} py-{}", spacing(Size::Xl), spacing(Size::Lg)),
        Size::Xl2 => format!("px-{} py-{}", spacing(Size::Xl2), spacing(Size::Xl)),
    };
}

// ------------------------------------------------------------------------------------------------
//  Boarders
// ------------------------------------------------------------------------------------------------

pub const ROUND_BORDER: &str = "rounded-lg";

#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(dead_code)]
pub enum BorderColor {
    Surface,
    Primary,
    Success,
    Warning,
    Error,
}

impl std::fmt::Display for BorderColor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let class = match self {
            BorderColor::Surface => "border border-surface-200 dark:border-surface-800",
            BorderColor::Primary => "border border-primary-500",
            BorderColor::Success => "border border-success",
            BorderColor::Warning => "border border-warning",
            BorderColor::Error => "border border-error",
        };
        return write!(f, "{}", class);
    }
}

pub fn standard_border(colour: Option<BorderColor>) -> String {
    let colour = colour.unwrap_or(BorderColor::Surface);
    return format!("{} {}", ROUND_BORDER, colour);
}

// ------------------------------------------------------------------------------------------------
//  Components
// ------------------------------------------------------------------------------------------------

#[component]
pub fn Stack(
    #[prop(default = Size::Sm)] size: Size,
    #[prop(default = false)] horizontal: bool,
    #[prop(default = true)] fill_space: bool,
    #[prop(default = FlexAlign::Stretch)] align: FlexAlign,
    #[prop(optional)] add_class: Option<String>,
    children: Children,
) -> impl IntoView {
    let additional_class = add_class.unwrap_or_default();

    let class_str = format!(
        "{} flex-{} gap-{} {} {}",
        if fill_space { "flex" } else { "inline-flex" },
        if horizontal { "row" } else { "col" },
        spacing(size),
        align,
        additional_class
    );
    return view! {
        <div class={class_str}>
            {children()}
        </div>
    };
}

#[component]
pub fn Grid(
    #[prop(default = Size::Sm)] size: Size,
    #[prop(default = 0)] cols: u8,
    #[prop(default = 0)] rows: u8,
    #[prop(default = true)] fill_space: bool,
    children: Children,
) -> impl IntoView {
    let col_str = if cols != 0 {
        format!("grid-cols-{}", cols)
    } else {
        "".to_string()
    };
    let row_str = if rows != 0 {
        format!("grid-rows-{}", rows)
    } else {
        "".to_string()
    };
    let class_str: String = format!(
        "{} {} {} gap-{}",
        if fill_space { "grid" } else { "inline-grid" },
        col_str,
        row_str,
        spacing(size)
    )
    .to_string();
    return view! {
        <div class={class_str}>
            {children()}
        </div>
    };
}

#[component]
pub fn BorderedDiv(
    #[prop(default = BorderColor::Surface)] border: BorderColor,
    #[prop(optional)] class: Option<String>,
    children: Children,
) -> impl IntoView {
    let additional_class = class.unwrap_or_default();

    view! {
        <div class=format!("overflow-hidden {} {}", standard_border(Some(border)), additional_class)>
            {children()}
        </div>
    }
}
