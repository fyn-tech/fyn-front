use leptos::prelude::*;
use crate::components::atoms::button::Button;
use crate::components::atoms::layout::*;
use crate::components::atoms::typography::*;

/// Component showcase page for design system development
#[component]
pub fn Showcase() -> impl IntoView {

    let color_boarder : String = "border border-surface-500 dark:border-surface-500".to_string();

    return view! {
        <div class="min-h-screen bg-surface-50 dark:bg-surface-950 transition-colors duration-200 p-8">
            <div class="max-w-6xl mx-auto">
                // Header with title and dark mode toggle
                <div class="flex justify-between items-center mb-8">
                    <H1>"Fynbos Design System"</H1>
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
                    <H2>"Design Foundation"</H2>
                    
                    // Color Palette Preview
                    <div class="mb-8 p-6 bg-surface-100 dark:bg-surface-800 rounded-lg border border-surface-200 dark:border-surface-700">
                        <H3>"Color Palette - Table Bay Blue & Emerald"</H3>
                        
                        <div class="grid grid-cols-1 md:grid-cols-4 gap-6">
                            // Primary Colors
                            <div>
                                <H4>"Primary (Table Bay Blue)"</H4>
                                <div class="flex gap-2">
                                    <div class=format!("w-12 h-12 bg-primary-50 rounded {}", color_boarder)></div>
                                    <div class=format!("w-12 h-12 bg-primary-500 rounded {}", color_boarder)></div>
                                    <div class=format!("w-12 h-12 bg-primary-950 rounded {}", color_boarder)></div>
                                </div>
                            </div>
                            
                            // Accent Colors
                            <div>
                                <H4>"Accent (Emerald)"</H4>
                                <div class="flex gap-2">
                                    <div class=format!("w-12 h-12 bg-accent-50 rounded {}", color_boarder)></div>
                                    <div class=format!("w-12 h-12 bg-accent-500 rounded {}", color_boarder)></div>
                                    <div class=format!("w-12 h-12 bg-accent-950 rounded {}", color_boarder)></div>
                                </div>
                            </div>
                            
                            // Surface Colors
                            <div>
                                <H4>"Surface (Table Mountain Grays)"</H4>
                                <div class="grid grid-cols-4 gap-1">
                                    <div class=format!("w-8 h-8 bg-surface-50 rounded {}", color_boarder)></div>
                                    <div class=format!("w-8 h-8 bg-surface-100 rounded {}", color_boarder)></div>
                                    <div class=format!("w-8 h-8 bg-surface-200 rounded {}", color_boarder)></div>
                                    <div class=format!("w-8 h-8 bg-surface-300 rounded {}", color_boarder)></div>
                                    // dark mode background move 'down', i.e. lower shade as go to cards etc.
                                    <div class=format!("w-8 h-8 bg-surface-950 rounded {}", color_boarder)></div>
                                    <div class=format!("w-8 h-8 bg-surface-900 rounded {}", color_boarder)></div>
                                    <div class=format!("w-8 h-8 bg-surface-800 rounded {}", color_boarder)></div>
                                    <div class=format!("w-8 h-8 bg-surface-700 rounded {}", color_boarder)></div>
                                </div>
                            </div>

                            // Sematic Colors
                            <div>
                                <H4>"Sematic"</H4>
                                <div class="flex gap-2">
                                    <div class=format!("w-12 h-12 bg-success rounded {}", color_boarder)></div>
                                    <div class=format!("w-12 h-12 bg-warning rounded {}", color_boarder)></div>
                                    <div class=format!("w-12 h-12 bg-error rounded {}", color_boarder)></div>
                                </div>
                            </div>
                        </div>
                    </div>
                </section>

                // Atoms section
                <section class="mb-12">
                    <H2>"Atoms"</H2>
                    
                    // Typography subsection
                    <div class="mb-8 p-6 bg-surface-100 dark:bg-surface-800 rounded-lg border border-surface-200 dark:border-surface-700">
                        <H3>"Typography"</H3>
                        <Stack>
                            <H1>"Heading 1"</H1>
                            <H2>"Heading 2"</H2>
                            <H3>"Heading 3"</H3>
                            <H4>"Heading 4"</H4>
                            <P>"Plain text"</P>
                        </Stack>
                    </div>

                    // Buttons subsection
                    <div class="mb-8 p-6 bg-surface-100 dark:bg-surface-800 rounded-lg border border-surface-200 dark:border-surface-700">
                        <H3>"Buttons"</H3>
                        <div class="flex gap-4 flex-wrap">
                            <Button />
                            <Button variant="secondary".to_string() text="Secondary".to_string() />
                            <Button text="Custom Text".to_string() />
                        </div>
                    </div>
                </section>
            </div>
        </div>
    };
}