use leptos::prelude::*;
use reqwest;

use crate::components::atoms::typography::P;
use crate::components::molecules::markdown_render::MarkdownRenderer;

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
