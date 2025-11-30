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
 * filename: fyn_tech_profile.rs
 * description: Company profile organism component
 * ------------------------------------------------------------------------------------------------
 */


use leptos::prelude::*;
use reqwest;

use crate::presentation::atoms::typography::P;
use crate::presentation::molecules::markdown_render::MarkdownRenderer;

#[component]
pub fn FynTechProfile() -> impl IntoView {
    let readme_resource = LocalResource::new(move || fetch_fyn_tech_readme());

    view! {
        {move || match readme_resource.get() {
            Some(Ok(content)) => view! { <MarkdownRenderer content={content} /> }.into_any(),
            Some(Err(error)) => view! { <P>{error}</P> }.into_any(),
            None => view! { <P>"Loading..."</P> }.into_any(),
          }
        }
    }
}

async fn fetch_fyn_tech_readme() -> Result<String, String> {
    reqwest::get("https://raw.githubusercontent.com/fyn-tech/.github/main/profile/README.md")
        .await
        .map_err(|e| format!("Network error: {}", e))?
        .text()
        .await
        .map_err(|e| format!("Text parsing error: {}", e))
}
