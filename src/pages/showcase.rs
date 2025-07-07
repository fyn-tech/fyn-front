use leptos::prelude::*;
use crate::components::atoms::button::Button; 

/// Component showcase page for design system development
#[component]
pub fn Showcase() -> impl IntoView {
    view! {
        <div class="min-h-screen bg-gray-50 p-8">
            <div class="max-w-6xl mx-auto">
                <h1 class="text-3xl font-bold text-gray-900 mb-8">
                    "Fynbos Design System"
                </h1>
                
                // Atoms section
                <section class="mb-12">
                    <h2 class="text-2xl font-semibold text-gray-800 mb-6">
                        "Atoms"
                    </h2>
                    
                    // Buttons subsection
                    <div class="mb-8">
                        <h3 class="text-lg font-medium text-gray-700 mb-4">
                            "Buttons"
                        </h3>

                        <div class="flex gap-4 flex-wrap">
                            <Button />
                            <Button variant="secondary".to_string() text="Secondary".to_string() />
                            <Button text="Custom Text".to_string() />
                        </div>
                    </div>
                </section>
            </div>
        </div>
    }
}