use leptos::prelude::*;
use crate::components::atoms::button::{*, Size as ButtonSize};
use crate::components::atoms::layout::*;
use crate::components::atoms::typography::*;
use crate::components::molecules::table::*;


/// Component showcase page for design system development
#[component]
pub fn Showcase() -> impl IntoView {

    return view! {
        <div class="min-h-screen bg-surface-50 dark:bg-surface-950 transition-colors duration-200 p-8">
            <div class="max-w-6xl mx-auto">
                // Header with title and dark mode toggle
                <div class="flex justify-between items-center mb-8">
                    <H1>"Fynbos Design System"</H1>
                    <Button 
                        variant=Variant::Secondary
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
                    <BorderedDiv class="mb-8 p-6 bg-surface-100 dark:bg-surface-900".to_string()>
                        <H3>"Color Palette - Table Bay Blue & Emerald"</H3>
                        
                        <Grid cols=3>
                            // Primary Colors
                            <Stack>
                                <H4>"Primary"</H4>
                                <Stack horizontal=true>
                                    <div class=format!("w-12 h-12 bg-primary-50 {}", standard_border(None))></div>
                                    <div class=format!("w-12 h-12 bg-primary-300 {}", standard_border(None))></div>
                                    <div class=format!("w-12 h-12 bg-primary-500 {}", standard_border(None))></div>
                                    <div class=format!("w-12 h-12 bg-primary-700 {}", standard_border(None))></div>
                                    <div class=format!("w-12 h-12 bg-primary-950 {}", standard_border(None))></div>
                                </Stack>
                            </Stack>
                            
                            // Accent Colors
                            <Stack>
                                <H4>"Accent"</H4>
                                <Stack horizontal=true>
                                    <div class=format!("w-12 h-12 bg-accent-50 {}", standard_border(None))></div>
                                    <div class=format!("w-12 h-12 bg-accent-300 {}", standard_border(None))></div>
                                    <div class=format!("w-12 h-12 bg-accent-500 {}", standard_border(None))></div>
                                    <div class=format!("w-12 h-12 bg-accent-700 {}", standard_border(None))></div>
                                    <div class=format!("w-12 h-12 bg-accent-950 {}", standard_border(None))></div>
                                </Stack>
                            </Stack>
                            
                            // Sematic Colors
                            <Stack>
                                <H4>"Sematic"</H4>
                                <Stack horizontal=true>
                                    <div class=format!("w-12 h-12 bg-semantic-success {}", standard_border(None))></div>
                                    <div class=format!("w-12 h-12 bg-semantic-warning {}", standard_border(None))></div>
                                    <div class=format!("w-12 h-12 bg-semantic-error {}", standard_border(None))></div>
                                </Stack>
                            </Stack>

                            // Surface Colors (Light)
                            <Stack>
                                <H4>"Surface (Light)"</H4>
                                <Stack horizontal=true>
                                    <div class=format!("w-12 h-12 bg-surface-50 {}", standard_border(None))></div>
                                    <div class=format!("w-12 h-12 bg-surface-100 {}", standard_border(None))></div>
                                    <div class=format!("w-12 h-12 bg-surface-200 {}", standard_border(None))></div>
                                    <div class=format!("w-12 h-12 bg-surface-300 {}", standard_border(None))></div>
                                </Stack>
                            </Stack>

                            // Surface Colors (Dark)
                            <Stack>
                                <H4>"Surface (Dark)"</H4>
                                <Stack horizontal=true>
                                    <div class=format!("w-12 h-12 bg-surface-950 {}", standard_border(None))></div>
                                    <div class=format!("w-12 h-12 bg-surface-900 {}", standard_border(None))></div>
                                    <div class=format!("w-12 h-12 bg-surface-800 {}", standard_border(None))></div>
                                    <div class=format!("w-12 h-12 bg-surface-700 {}", standard_border(None))></div>
                                </Stack>
                            </Stack>
                        </Grid>
                    </BorderedDiv>
                
                    <H2>"Atoms"</H2>
                    
                    // Typography subsection
                    <BorderedDiv class="mb-8 p-6 bg-surface-100 dark:bg-surface-900".to_string()>
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
                    </BorderedDiv>

                    // Buttons subsection
                    <BorderedDiv class="mb-8 p-6 bg-surface-100 dark:bg-surface-900".to_string()>
                        <H3>"Button Variants"</H3>
                        <Stack horizontal=true>
                            <Button variant=Variant::Primary text="Primary".to_string() />
                            <Button variant=Variant::Secondary text="Secondary".to_string() />
                            <Button variant=Variant::Tertiary text="Tertiary".to_string() />
                            <Button variant=Variant::Warning text="Warning".to_string() />
                            <Button variant=Variant::Success text="Success".to_string() />
                        </Stack>
                        
                        <H3>"Button States"</H3>
                        <Stack horizontal=true>
                            <Button variant=Variant::Primary state=State::Default text="Default".to_string() />
                            <Button variant=Variant::Primary state=State::Active text="Active".to_string() />
                            <Button variant=Variant::Primary state=State::Disabled text="Disabled".to_string() />
                            <Button variant=Variant::Primary state=State::Loading text="Loading".to_string() />
                        </Stack>

                        <H3>"Button Sizes"</H3>
                        <Stack horizontal=true align=FlexAlign::End>
                            <Button variant=Variant::Primary size=ButtonSize::Xl text="Xl".to_string() />
                            <Button variant=Variant::Primary size=ButtonSize::Lg text="Lg".to_string() />
                            <Button variant=Variant::Primary size=ButtonSize::Md text="Md".to_string() />
                            <Button variant=Variant::Primary size=ButtonSize::Sm text="Sm".to_string() />
                            <Button variant=Variant::Primary size=ButtonSize::Xs text="Xs".to_string() />
                        </Stack>
                    </BorderedDiv>

                    <H2>"Molecules"</H2>          

                     // Tables
                    <BorderedDiv class="mb-8 p-6 bg-surface-100 dark:bg-surface-900".to_string()>
                        <H3>"Tables"</H3>
                        <Table table={get_example_table_data()}></Table>
                    </BorderedDiv>
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