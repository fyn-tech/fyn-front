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
 * filename: section.rs
 * description: Section molecule component
 * ------------------------------------------------------------------------------------------------
 */


use leptos::prelude::*;

use crate::common::size::*;
use crate::components::atoms::layout::{spacing, Align, Stack};
use crate::components::atoms::typography::{H1, H2, H3, H4};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SectionLevel {
    H1,
    H2,
    H3,
    H4,
}

#[component]
pub fn Section(
    #[prop(default = false)] is_first: bool,
    #[prop(default = false)] centre: bool,
    #[prop(default = false)] spaced: bool,
    level: SectionLevel,
    title: String,
    children: Children,
) -> impl IntoView {
    let space = if !is_first {
        match level {
            SectionLevel::H1 => spacing(Size::Xl),
            SectionLevel::H2 => spacing(Size::Lg),
            SectionLevel::H3 => spacing(Size::Md),
            SectionLevel::H4 => spacing(Size::Sm),
        }
    } else {
        spacing(Size::None)
    };

    let class_str = format!("mt-{} mb-{}", space, space);
    let align = if centre { Align::Center } else { Align::Left };

    let heading = match level {
        SectionLevel::H1 => view! { <H1 align={align}>{title.clone()}</H1> }.into_any(),
        SectionLevel::H2 => view! { <H2 align={align}>{title.clone()}</H2> }.into_any(),
        SectionLevel::H3 => view! { <H3 align={align}>{title.clone()}</H3> }.into_any(),
        SectionLevel::H4 => view! { <H4 align={align}>{title.clone()}</H4> }.into_any(),
    };

    view! {
        <Stack size={if spaced { Size::Lg } else { Size::Sm }} add_class={class_str}>
            {heading}
            <Stack>
                {children()}
            </Stack>
        </Stack>
    }
}
