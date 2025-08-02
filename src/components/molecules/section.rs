use leptos::prelude::*;

use crate::common::size::*;
use crate::components::atoms::layout::{spacing, Stack};
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
    level: SectionLevel,
    title: String,
    children: Children,
) -> impl IntoView {
    let space_above = if !is_first {
        match level {
            SectionLevel::H1 => spacing(Size::Xl),
            SectionLevel::H2 => spacing(Size::Lg),
            SectionLevel::H3 => spacing(Size::Md),
            SectionLevel::H4 => spacing(Size::Sm),
        }
    } else {
        spacing(Size::None)
    };

    let heading = match level {
        SectionLevel::H1 => view! { <H1>{title.clone()}</H1> }.into_any(),
        SectionLevel::H2 => view! { <H2>{title.clone()}</H2> }.into_any(),
        SectionLevel::H3 => view! { <H3>{title.clone()}</H3> }.into_any(),
        SectionLevel::H4 => view! { <H4>{title.clone()}</H4> }.into_any(),
    };

    view! {
        <div class=format!("mt-{} mt-{}", space_above, space_above)>
            {heading}
            <Stack>
                {children()}
            </Stack>
        </div>
    }
}
