use leptos::prelude::*;
use crate::components::atoms::button::Button;

/// Component showcase page for design system development
#[component]
pub fn Showcase() -> impl IntoView {
    view! {
        <div class="min-h-screen bg-surface-50 dark:bg-surface-900 transition-colors duration-200 p-8">
            <div class="max-w-6xl mx-auto">
                // Header with title and dark mode toggle
                <div class="flex justify-between items-center mb-8">
                    <h1 class="text-3xl font-bold text-content-primary dark:text-content-primary-dark">
                        "Fynbos Design System"
                    </h1>
                    
                    <Button 
                        variant="secondary".to_string() 
                        text="Toggle Dark Mode".to_string()
                        on_click=Box::new(|| {
                            let window = web_sys::window().expect("should have a window");
                            let document = window.document().expect("should have a document");
                            let html_element = document.document_element().expect("should have html element");
                            
                            if html_element.class_list().contains("dark") {
                                html_element.class_list().remove_1("dark").expect("should remove dark class");
                            } else {
                                html_element.class_list().add_1("dark").expect("should add dark class");
                            }
                        })
                    />
                </div>
                
                <section class="mb-12">
                    <h2 class="text-2xl font-semibold text-content-primary dark:text-content-primary-dark mb-6">
                        "Design Foundation"
                    </h2>
                    

                    // Color Palette Preview
                    <div class="mb-8 p-6 bg-surface-100 dark:bg-surface-800 rounded-lg border border-surface-200 dark:border-surface-700">
                        <h3 class="text-lg font-medium text-content-secondary dark:text-content-secondary-dark mb-4">
                            "Color Palette - Table Bay Blue & Emerald"
                        </h3>
                        
                        <div class="grid grid-cols-1 md:grid-cols-4 gap-6">
                            // Primary Colors
                            <div>
                                <h4 class="text-sm font-medium text-content-tertiary dark:text-content-tertiary-dark mb-2">
                                    "Primary (Table Bay Blue)"
                                </h4>
                                <div class="flex gap-2">
                                    <div class="w-12 h-12 bg-primary-50 rounded border border-surface-200 dark:border-surface-600"></div>
                                    <div class="w-12 h-12 bg-primary-500 rounded"></div>
                                    <div class="w-12 h-12 bg-primary-900 rounded"></div>
                                </div>
                            </div>
                            
                            // Accent Colors
                            <div>
                                <h4 class="text-sm font-medium text-content-tertiary dark:text-content-tertiary-dark mb-2">
                                    "Accent (Emerald)"
                                </h4>
                                <div class="flex gap-2">
                                    <div class="w-12 h-12 bg-accent-50 rounded border border-surface-200 dark:border-surface-600"></div>
                                    <div class="w-12 h-12 bg-accent-500 rounded"></div>
                                    <div class="w-12 h-12 bg-accent-900 rounded"></div>
                                </div>
                            </div>
                            
                            // Surface Colors
                            <div>
                                <h4 class="text-sm font-medium text-content-tertiary dark:text-content-tertiary-dark mb-2">
                                    "Surface (Table Mountain Grays)"
                                </h4>
                                <div class="grid grid-cols-4 gap-1">
                                    <div class="w-8 h-8 bg-surface-50 rounded border border-surface-200 dark:border-surface-600"></div>
                                    <div class="w-8 h-8 bg-surface-100 rounded border border-surface-200 dark:border-surface-600"></div>
                                    <div class="w-8 h-8 bg-surface-200 rounded"></div>
                                    <div class="w-8 h-8 bg-surface-300 rounded"></div>
                                    <div class="w-8 h-8 bg-surface-600 rounded"></div>
                                    <div class="w-8 h-8 bg-surface-700 rounded"></div>
                                    <div class="w-8 h-8 bg-surface-800 rounded"></div>
                                    <div class="w-8 h-8 bg-surface-900 rounded"></div>
                                </div>
                            </div>

                            // Sematic Colors
                            <div>
                                <h4 class="text-sm font-medium text-content-tertiary dark:text-content-tertiary-dark mb-2">
                                    "Sematic"
                                </h4>
                                <div class="flex gap-2">
                                    <div class="w-12 h-12 bg-success rounded border border-surface-200 dark:border-surface-600"></div>
                                    <div class="w-12 h-12 bg-warning rounded"></div>
                                    <div class="w-12 h-12 bg-error rounded"></div>
                                </div>
                            </div>
                        </div>
                    </div>
                </section>

                // Atoms section
                <section class="mb-12">
                    <h2 class="text-2xl font-semibold text-content-primary dark:text-content-primary-dark mb-6">
                        "Atoms"
                    </h2>
                    
                    // Buttons subsection
                    <div class="mb-8 p-6 bg-surface-100 dark:bg-surface-800 rounded-lg border border-surface-200 dark:border-surface-700">
                        <h3 class="text-lg font-medium text-content-secondary dark:text-content-secondary-dark mb-4">
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