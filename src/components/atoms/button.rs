use leptos::prelude::*;

#[component]
pub fn Button(
    #[prop(default = "primary".to_string())] variant: String,
    #[prop(default = "Click me".to_string())] text: String,
) -> impl IntoView {
    let button_classes = match variant.as_str() {
        "primary" => "bg-primary-500 hover:bg-primary-900 text-white px-4 py-2 rounded",
        "secondary" => "bg-gray-200 hover:bg-gray-300 text-gray-800 px-4 py-2 rounded",
        _ => "bg-primary-500 hover:bg-primary-900 text-white px-4 py-2 rounded",
    };

    view! {
        <button class={button_classes}>
            {text}
        </button>
    }
}