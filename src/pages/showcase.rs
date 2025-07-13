use leptos::prelude::*;
use crate::components::atoms::button::Button;
use crate::components::atoms::layout::*;
use crate::components::atoms::typography::*;
use crate::components::molecules::table::*;


/// Component showcase page for design system development
#[component]
pub fn Showcase() -> impl IntoView {

    let color_boarder = "border border-surface-500 dark:border-surface-500";

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
                
                <Stack>
                    <H2>"Design Foundation"</H2>
                        
                    // Color Palette Preview
                    <div class="mb-8 p-6 bg-surface-100 dark:bg-surface-900 rounded-lg border border-surface-200 dark:border-surface-800">
                        <H3>"Color Palette - Table Bay Blue & Emerald"</H3>
                        
                        <Grid cols=3>
                            // Primary Colors
                            <Stack>
                                <H4>"Primary"</H4>
                                <Stack horizontal=true>
                                    <div class=format!("w-12 h-12 bg-primary-50 rounded {}", color_boarder)></div>
                                    <div class=format!("w-12 h-12 bg-primary-500 rounded {}", color_boarder)></div>
                                    <div class=format!("w-12 h-12 bg-primary-950 rounded {}", color_boarder)></div>
                                </Stack>
                            </Stack>
                            
                            // Accent Colors
                            <Stack>
                                <H4>"Accent"</H4>
                                <Stack horizontal=true>
                                    <div class=format!("w-12 h-12 bg-accent-50 rounded {}", color_boarder)></div>
                                    <div class=format!("w-12 h-12 bg-accent-500 rounded {}", color_boarder)></div>
                                    <div class=format!("w-12 h-12 bg-accent-950 rounded {}", color_boarder)></div>
                                </Stack>
                            </Stack>
                            
                            // Sematic Colors
                            <Stack>
                                <H4>"Sematic"</H4>
                                <Stack horizontal=true>
                                    <div class=format!("w-12 h-12 bg-success rounded {}", color_boarder)></div>
                                    <div class=format!("w-12 h-12 bg-warning rounded {}", color_boarder)></div>
                                    <div class=format!("w-12 h-12 bg-error rounded {}", color_boarder)></div>
                                </Stack>
                            </Stack>

                            // Surface Colors (Light)
                            <Stack>
                                <H4>"Surface (Light)"</H4>
                                <Stack horizontal=true>
                                    <div class=format!("w-12 h-12 bg-surface-50 rounded {}", color_boarder)></div>
                                    <div class=format!("w-12 h-12 bg-surface-100 rounded {}", color_boarder)></div>
                                    <div class=format!("w-12 h-12 bg-surface-200 rounded {}", color_boarder)></div>
                                    <div class=format!("w-12 h-12 bg-surface-300 rounded {}", color_boarder)></div>
                                </Stack>
                            </Stack>

                            // Surface Colors (Dark)
                            <Stack>
                                <H4>"Surface (Dark)"</H4>
                                <Stack horizontal=true>
                                    <div class=format!("w-12 h-12 bg-surface-950 rounded {}", color_boarder)></div>
                                    <div class=format!("w-12 h-12 bg-surface-900 rounded {}", color_boarder)></div>
                                    <div class=format!("w-12 h-12 bg-surface-800 rounded {}", color_boarder)></div>
                                    <div class=format!("w-12 h-12 bg-surface-700 rounded {}", color_boarder)></div>
                                </Stack>
                            </Stack>
                        </Grid>
                    </div>
                
                    <H2>"Atoms"</H2>
                    
                    // Typography subsection
                    <div class="mb-8 p-6 bg-surface-100 dark:bg-surface-900 rounded-lg border border-surface-200 dark:border-surface-800">
                        <H3>"Typography"</H3>
                        <Stack>
                            <H1>"Heading 1"</H1>
                            <H2>"Heading 2"</H2>
                            <H3>"Heading 3"</H3>
                            <H4>"Heading 4"</H4>
                            <P>
                                "Plain text paragraph: The computational fluid dynamics simulation converged after 1,247 iterations using a k-epsilon turbulence model with enhanced wall treatment. 
                                Pressure distribution analysis revealed significant flow separation downstream of the airfoil at angles of attack exceeding 15 degrees.
                                The Reynolds number of 2.3 × 10⁶ produced transitional boundary layer behavior, requiring adaptive mesh refinement near the leading edge to capture laminar-to-turbulent transition accurately.
                                Velocity magnitude contours demonstrated the expected acceleration over the upper surface, with maximum velocities reaching 1.8 times the freestream value.
                                Post-processing results indicate that the drag coefficient increased nonlinearly with angle of attack, while lift coefficient exhibited the characteristic stall behavior at 18 degrees. 
                                These findings validate the numerical methodology for future parametric studies involving geometric optimization of the airfoil section."
                            </P>
                        </Stack>
                    </div>

                    // Buttons subsection
                    <div class="mb-8 p-6 bg-surface-100 dark:bg-surface-900 rounded-lg border border-surface-200 dark:border-surface-800">
                        <H3>"Buttons"</H3>
                        <Stack horizontal=true>
                            <Button />
                            <Button variant="secondary".to_string() text="Secondary".to_string() />
                            <Button text="Custom Text".to_string() />
                        </Stack>
                    </div>

                     <H2>"Molecules"</H2>
                        
                     // Tables
                     <div class="mb-8 p-6 bg-surface-100 dark:bg-surface-900 rounded-lg border border-surface-200 dark:border-surface-800">
                        <H3>"Tables"</H3>
                        <Table table={get_example_table_data()}></Table>
                     </div>
                </Stack>
            </div>
        </div>
    };
}

fn get_example_table_data() -> TableStruct{
    return  TableStruct {
        name: "Airfoil Analysis Results".to_string(),
        data: TableData {
            col_def: vec![
                ColumnDefinition { 
                    name: "Parameter".to_string(), 
                    data_type: CellType::Text 
                },
                ColumnDefinition { 
                    name: "Value".to_string(), 
                    data_type: CellType::Float 
                },
                ColumnDefinition { 
                    name: "Units".to_string(), 
                    data_type: CellType::Text 
                },
                ColumnDefinition { 
                    name: "Iterations".to_string(), 
                    data_type: CellType::Int 
                },
                ColumnDefinition { 
                    name: "Status".to_string(), 
                    data_type: CellType::Text 
                },
            ],
            rows: vec![
                vec!["Drag Coefficient".to_string(), "0.0234".to_string(), "-".to_string(), "1247".to_string(), "Converged".to_string()],
                vec!["Lift Coefficient".to_string(), "1.2456".to_string(), "-".to_string(), "1247".to_string(), "Converged".to_string()],
                vec!["Pressure Drop".to_string(), "1245.67".to_string(), "Pa".to_string(), "892".to_string(), "Converged".to_string()],
                vec!["Reynolds Number".to_string(), "2300000.0".to_string(), "-".to_string(), "0".to_string(), "Input".to_string()],
                vec!["Angle of Attack".to_string(), "15.5".to_string(), "degrees".to_string(), "0".to_string(), "Input".to_string()],
                vec!["Mach Number".to_string(), "0.3".to_string(), "-".to_string(), "0".to_string(), "Input".to_string()],
                vec!["Turbulence Intensity".to_string(), "0.05".to_string(), "%".to_string(), "1450".to_string(), "Converged".to_string()],
                vec!["Wall Y+".to_string(), "1.2".to_string(), "-".to_string(), "2100".to_string(), "Converged".to_string()],
            ],
        },
    };
}