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
 * filename: showcase.rs
 * description: Showcase/demo page component
 * ------------------------------------------------------------------------------------------------
 */

use leptos::prelude::*;

use crate::common::size::*;
use crate::components::atoms::alert::*;
use crate::components::atoms::button::*;
use crate::components::atoms::input::*;
use crate::components::atoms::layout::*;
use crate::components::atoms::typography::{H1, H2, H3, H4, P};
use crate::components::molecules::button_bar::*;
use crate::components::molecules::form_field::*;
use crate::components::molecules::section::*;
use crate::components::molecules::table::*;

/// Component showcase page for design system development
#[component]
pub fn Showcase() -> impl IntoView {
    let dummy_text_signal = RwSignal::new(String::new());
    let dummy_float_signal = RwSignal::new(None);
    let dummy_integer_signal = RwSignal::new(None);
    let dummy_email_signal = RwSignal::new(String::new());
    let dummy_bool_signal = RwSignal::new(false);
    let dummy_password_signal = RwSignal::new(String::new());
    let dummy_file_signal = RwSignal::new(String::new());

    return view! {
        <div class="min-h-screen bg-surface-50 dark:bg-surface-950 transition-colors duration-200 p-8">
            <div class="max-w-6xl mx-auto">
                // Header with title and dark mode toggle
                <div class="flex justify-between items-center mb-8">
                    <H1>"Fynbos Design System"</H1>
                    <Button button_data=
                        ButtonData::new()
                        .variant(Variant::Secondary)
                        .text("Toggle Dark Mode")
                         .on_click(Box::new(|| {
                             let window = web_sys::window().expect("should have a window");
                             let document = window.document().expect("should have a document");
                             let html_element = document.document_element().expect("should have html element");
                             if html_element.class_list().contains("dark") {
                                 html_element.class_list().remove_1("dark").expect("should remove dark class");
                             } else {
                                 html_element.class_list().add_1("dark").expect("should add dark class");
                             }
                         }))

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

                    <BorderedDiv class="mb-8 p-6 bg-surface-100 dark:bg-surface-900".to_string()>
                        <H3>"Alerts"</H3>
                        <Stack>
                            <InfoAlert message={RwSignal::new(Some("InfoAlert".to_string())).read_only()} />
                            <SuccessAlert message={RwSignal::new(Some("SuccessAlert".to_string())).read_only()} />
                            <WarningAlert message={RwSignal::new(Some("WarningAlert".to_string())).read_only()} />
                            <ErrorAlert message={RwSignal::new(Some("ErrorAlert".to_string())).read_only()} />
                        </Stack>
                    </BorderedDiv>

                    // Buttons subsection
                    <BorderedDiv class="mb-8 p-6 bg-surface-100 dark:bg-surface-900".to_string()>
                        <Section level=SectionLevel::H3 is_first=true title="Button Variants".to_string()>
                            <Stack horizontal=true>
                                <Button button_data=ButtonData::new().variant(Variant::Primary).text("Primary") />
                                <Button button_data=ButtonData::new().variant(Variant::Secondary).text("Secondary") />
                                <Button button_data=ButtonData::new().variant(Variant::Tertiary).text("Tertiary") />
                            </Stack>
                        </Section>

                        <Section level=SectionLevel::H3 title="Button States".to_string()>
                            <Stack horizontal=true>
                                <Button button_data=ButtonData::new().text("Default") />
                                <Button button_data=ButtonData::new().state(State::Active).text("Active") />
                                <Button button_data=ButtonData::new().state(State::Disabled).text("Disabled") />
                                <Button button_data=ButtonData::new().state(State::Loading).text("Loading") />
                                <Button button_data=ButtonData::new().state(State::Success).text("Success") />
                                <Button button_data=ButtonData::new().state(State::Error).text("Error") />
                            </Stack>
                        </Section>

                        <Section level=SectionLevel::H3 title="Button Sizes".to_string()>
                            <Stack horizontal=true align=FlexAlign::End>
                                <Button button_data=ButtonData::new().size(Size::Xl2).text("Xl2") />
                                <Button button_data=ButtonData::new().size(Size::Xl).text("Xl") />
                                <Button button_data=ButtonData::new().size(Size::Lg).text("Lg") />
                                <Button button_data=ButtonData::new().size(Size::Md).text("Md") />
                                <Button button_data=ButtonData::new().size(Size::Sm).text("Sm") />
                                <Button button_data=ButtonData::new().size(Size::Xs).text("Xs") />
                            </Stack>
                        </Section>
                    </BorderedDiv>

                    <H2>"Form Elements"</H2>
                    <BorderedDiv class="mb-8 p-6 bg-surface-100 dark:bg-surface-900".to_string()>
                        <Section level=SectionLevel::H3 is_first=true title="Text Input".to_string()>
                            <Stack horizontal=true align=FlexAlign::Center>
                                <Text id="text".to_string() key="text".to_string() signal=dummy_text_signal/>
                            </Stack>
                        </Section>

                        <Section level=SectionLevel::H3 title="Numerical".to_string()>
                            <Stack horizontal=true align=FlexAlign::Center>
                                <Float id="float".to_string() key="float".to_string() signal=dummy_float_signal/>
                                <Integer id="int".to_string() key="int".to_string() signal=dummy_integer_signal/>
                            </Stack>
                        </Section>

                        <Section level=SectionLevel::H3 title="Email & Password".to_string()>
                            <Stack horizontal=true align=FlexAlign::Center>
                                <Email id="email".to_string() key="email".to_string() signal=dummy_email_signal/>
                                <Password id="password".to_string() key="password".to_string() signal=dummy_password_signal/>
                            </Stack>
                        </Section>

                        <Section level=SectionLevel::H3 title="File".to_string()>
                            <Stack horizontal=true align=FlexAlign::Center>
                                <File id="file".to_string() key="file".to_string() signal=dummy_file_signal/>
                            </Stack>
                        </Section>

                        <Section level=SectionLevel::H3 title="Check Box".to_string()>
                            <Stack horizontal=true align=FlexAlign::Center>
                                <CheckBox id="checkbox".to_string() key="checkbox".to_string() signal=dummy_bool_signal/>
                            </Stack>
                        </Section>

                        <Section level=SectionLevel::H3 title="Select".to_string()>
                            <Stack horizontal=true align=FlexAlign::Center>
                                <SelectText id="select".to_string() key="select".to_string()
                                signal={dummy_text_signal}
                                options={vec![
                                    ("item_1".to_string(), "item 1".to_string()),
                                    ("item_2".to_string(), "item 2".to_string()),
                                    ]}/>
                            </Stack>
                        </Section>
                    </BorderedDiv>

                    <H2>"Molecules"</H2>
                     // Button Bars
                     <BorderedDiv class="mb-8 p-6 bg-surface-100 dark:bg-surface-900".to_string()>
                        <Section level=SectionLevel::H3 is_first=true title="Button Bars".to_string()>
                            <Grid cols=1 fill_space=false>
                                <ButtonBar items=vec![
                                    view! {<GroupButton button_data=ButtonData::new().text("First") />},
                                    view! {<GroupButton button_data=ButtonData::new().text("Second") />},
                                    view! {<GroupButton button_data=ButtonData::new().text("Third") />}
                                ]/>
                                <ButtonBar horizontal=false items=vec![
                                    view! {<GroupButton button_data=ButtonData::new().text("First") />},
                                    view! {<GroupButton button_data=ButtonData::new().text("Second") />},
                                    view! {<GroupButton button_data=ButtonData::new().text("Third") />}
                                ]/>
                            </Grid>
                        </Section>
                     </BorderedDiv>

                    <BorderedDiv class="mb-8 p-6 bg-surface-100 dark:bg-surface-900".to_string()>
                        <Section level=SectionLevel::H3 is_first=true title="Form Fields Horizontal".to_string()>
                            <div class="w-1/2"> // would normally be done by the form
                                <Stack>
                                    <FormField
                                        label="Text Field".to_string()
                                        key="form_text".to_string()
                                        input_type=InputType::Text { signal: dummy_text_signal}>
                                    </FormField>
                                    <FormField
                                        label="Select Field".to_string()
                                        key="form_select".to_string()
                                        input_type=InputType::SelectText {
                                            signal:{dummy_text_signal},
                                            options:{vec![
                                            ("item_1".to_string(), "item 1".to_string()),
                                            ("item_2".to_string(), "item 2".to_string()),
                                            ]}
                                        }>
                                    </FormField>
                                    <FormField
                                        label="Number Field".to_string()
                                        key="form_number".to_string()
                                        input_type=InputType::Float {
                                            signal: dummy_float_signal,
                                            min: (None),
                                            max: (None),
                                            step: (None) }>
                                        </FormField>
                                </Stack>
                            </div>
                        </Section>

                        <Section level=SectionLevel::H3 title="Vertical Fields Horizontal".to_string()>
                            <div class="w-1/2"> // would normally be done by the form
                                <Stack>
                                    <FormField
                                        label="Text Field".to_string()
                                        key="form_text_v".to_string()
                                        horizontal=false
                                        input_type=InputType::Text { signal: dummy_text_signal }>
                                    </FormField>
                                    <FormField
                                        label="Select Field".to_string()
                                        key="form_select_v".to_string()
                                        horizontal=false
                                        input_type=InputType::SelectText {
                                            signal:{dummy_text_signal},
                                            options:{vec![
                                            ("item_1".to_string(), "item 1".to_string()),
                                            ("item_2".to_string(), "item 2".to_string()),
                                            ]}}>
                                    </FormField>
                                    <FormField
                                        label="Number Field".to_string()
                                        key="form_number_v".to_string()
                                        horizontal=false
                                        input_type=InputType::Float {
                                            signal: dummy_float_signal,
                                            min: (None),
                                            max: (None),
                                            step: (None) }>
                                        </FormField>
                                </Stack>
                            </div>
                        </Section>
                    </BorderedDiv>

                     // Tables
                    <BorderedDiv class="mb-8 p-6 bg-surface-100 dark:bg-surface-900".to_string()>
                        <Section level=SectionLevel::H3 is_first=true title="Tables".to_string()>
                            <Table table={get_example_table_data()}></Table>
                        </Section>
                    </BorderedDiv>
                </Stack>
            </div>
        </div>
    };
}

fn get_example_table_data() -> TableStruct {
    return TableStruct {
        name: "Airfoil Analysis Results".to_string(),
        data: TableData {
            col_def: vec![
                ColumnDefinition {
                    name: "Parameter".to_string(),
                    data_type: CellType::Text,
                },
                ColumnDefinition {
                    name: "Value".to_string(),
                    data_type: CellType::Float,
                },
                ColumnDefinition {
                    name: "Units".to_string(),
                    data_type: CellType::Text,
                },
                ColumnDefinition {
                    name: "Iterations".to_string(),
                    data_type: CellType::Int,
                },
                ColumnDefinition {
                    name: "Status".to_string(),
                    data_type: CellType::Text,
                },
            ],
            rows: vec![
                vec![
                    "Drag Coefficient".to_string(),
                    "0.0234".to_string(),
                    "-".to_string(),
                    "1247".to_string(),
                    "Converged".to_string(),
                ],
                vec![
                    "Lift Coefficient".to_string(),
                    "1.2456".to_string(),
                    "-".to_string(),
                    "1247".to_string(),
                    "Converged".to_string(),
                ],
                vec![
                    "Pressure Drop".to_string(),
                    "1245.67".to_string(),
                    "Pa".to_string(),
                    "892".to_string(),
                    "Converged".to_string(),
                ],
                vec![
                    "Reynolds Number".to_string(),
                    "2300000.0".to_string(),
                    "-".to_string(),
                    "0".to_string(),
                    "Input".to_string(),
                ],
                vec![
                    "Angle of Attack".to_string(),
                    "15.5".to_string(),
                    "degrees".to_string(),
                    "0".to_string(),
                    "Input".to_string(),
                ],
                vec![
                    "Mach Number".to_string(),
                    "0.3".to_string(),
                    "-".to_string(),
                    "0".to_string(),
                    "Input".to_string(),
                ],
                vec![
                    "Turbulence Intensity".to_string(),
                    "0.05".to_string(),
                    "%".to_string(),
                    "1450".to_string(),
                    "Converged".to_string(),
                ],
                vec![
                    "Wall Y+".to_string(),
                    "1.2".to_string(),
                    "-".to_string(),
                    "2100".to_string(),
                    "Converged".to_string(),
                ],
            ],
        },
    };
}
