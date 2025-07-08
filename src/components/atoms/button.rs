use leptos::prelude::*;

#[component]
pub fn Button(
    #[prop(default = "primary".to_string())] variant: String,
    #[prop(default = "Click me".to_string())] text: String,
    #[prop(optional)] on_click: Option<Box<dyn Fn() + 'static>>,
) -> impl IntoView {
    let button_classes = match variant.as_str() {
        "primary" => "bg-primary-500 hover:bg-primary-600 dark:bg-primary-600 dark:hover:bg-primary-700 text-white px-4 py-2 rounded transition-colors duration-200",
        "secondary" => "bg-surface-200 hover:bg-surface-300 dark:bg-surface-700 dark:hover:bg-surface-600 text-content-primary dark:text-content-primary-dark px-4 py-2 rounded transition-colors duration-200 border border-surface-300 dark:border-surface-600",
        _ => "bg-primary-500 hover:bg-primary-600 dark:bg-primary-600 dark:hover:bg-primary-700 text-white px-4 py-2 rounded transition-colors duration-200",
    };

    view! {
        <button 
            class={button_classes}
            on:click=move |_| {
                if let Some(ref action) = on_click {
                    action();
                }
            }
        >
            {text}
        </button>
    }
}