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

use leptos::prelude::*;
use std::collections::HashMap;
use uuid::Uuid;

use crate::domain::runner_info::RunnerInfo;
use crate::presentation::molecules::table::*;

// use crate::domain::user_context::UserContext;

#[component]
fn RunnerInstallView() -> impl IntoView {
    view! {<a>"Hello".to_string()</a>}
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
