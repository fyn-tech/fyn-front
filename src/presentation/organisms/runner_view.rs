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
 * filename: runner_form.rs
 * description: View of the user context.
 * ------------------------------------------------------------------------------------------------
 */

use leptos::{prelude::*, reactive::spawn_local};
use std::collections::HashMap;
use uuid::Uuid;

use crate::common::size::*;
use crate::domain::runner_info::RunnerInfo;
use crate::infrastructure::fyn_api_client::FynApiClient;
use crate::presentation::atoms::alert::*;
use crate::presentation::atoms::button::*;
use crate::presentation::atoms::layout::*;
use crate::presentation::atoms::typography::*;
use crate::presentation::molecules::form_field::*;
use crate::presentation::molecules::section::*;
use crate::presentation::molecules::table::*;
use crate::presentation::view_models::new_runner_model::*;
// use crate::domain::user_context::UserContext;

fn handle_new_runner_install(runner_form: &NewRunnerModel) {
    let fyn_api_client: FynApiClient =
        use_context::<FynApiClient>().expect("FynApiClient should be provided");

    let cloned_runner_form = runner_form.clone();
    spawn_local(async move {
        let response = fyn_api_client
            .create_runner(cloned_runner_form.clone().into())
            .await;
        match response {
            Ok(response) => cloned_runner_form.set_info(
                format!("\
                Success, new runner created! 
                \nDuring first time runner startup you will be requested to provide the runner ID \
                and Registration Token below. Do not navigate away until you have done this, the token is not recoverable.
                \nRunner ID:          {}
                Registration Token: {}
                ", response.id, response.token.unwrap()
            )),
            Err(msg) => cloned_runner_form.set_error(msg),
        }
    });
}

#[component]
fn RunnerInstallView() -> impl IntoView {
    let new_runner = NewRunnerModel::new();
    view! {
        <form on:submit=|e| e.prevent_default()>
            <Section level={SectionLevel::H1}
            centre={true}
            spaced={true}
            max_width={Size::Xl4}

            title={"Resgister New Runner".to_string()}>

                <H2>"Runner Installation"</H2>
                <P>
                "To install the fyn-tech runner you are asked to download it from source from githup at the below link
                 and follow the instructions."
                </P>
                <H2>"New Runner Details"</H2>
                <Grid size={Size::Xl} cols=1>
                    <FormField
                        label={"Runner Name".to_string()}
                        key={"name".to_string()}
                        input_type=InputType::Text { signal: new_runner.name }
                    />
                </Grid>
                <ErrorAlert message={new_runner.error.read_only()} />
                <InfoAlert message={new_runner.info.read_only()} />
                <Stack align=FlexAlign::Center size=Size::Md add_class="py-4".to_string()>
                    <Button button_data=ButtonData::new()
                    .text("Create New Runner")
                    .on_click(Box::new(move || handle_new_runner_install(&new_runner)))
                    />
                </Stack>
            </Section>
        </form>
    }
}

#[component]
pub fn RunnerView(runners: Option<HashMap<Uuid, RunnerInfo>>) -> impl IntoView {
    view! {
        {move || {
            match &runners {
                Some(runner_map) => {
                    if runner_map.len() == 0 {
                        return view!{
                            <RunnerInstallView/>
                        }.into_any();
                    }

                    let rows = runner_map.iter().map(|(_, runner)| {
                        vec![
                            runner.name.clone(),
                            format!("{:?}", runner.state),
                            runner.last_contact
                                .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
                                .unwrap_or_else(|| "Never".to_string()),
                            runner.created_at.format("%Y-%m-%d %H:%M:%S").to_string(),
                            runner.id.to_string(),
                        ]
                    }).collect::<Vec<Vec<String>>>();

                    view! {
                        <Table table={TableStruct {
                            name: "Runner List".to_string(),
                            data: TableData {
                                col_def: vec![
                                    ColumnDefinition {
                                        name: "Name".to_string(),
                                        data_type: CellType::Text
                                    },
                                    ColumnDefinition {
                                        name: "Status".to_string(),
                                        data_type: CellType::Text
                                    },
                                    ColumnDefinition {
                                        name: "Last Contact".to_string(),
                                        data_type: CellType::Text
                                    },
                                    ColumnDefinition {
                                        name: "Created".to_string(),
                                        data_type: CellType::Text
                                    },
                                    ColumnDefinition {
                                        name: "ID".to_string(),
                                        data_type: CellType::Text
                                    },
                                ],
                                rows
                            }
                        }}/>
                    }.into_any()
                },
                None => view! { <div>"Loading runners..."</div> }.into_any()
            }
        }}
    }
}
