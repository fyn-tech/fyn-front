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
 * filename: job_manager_view.rs
 * description: Job manager view
 * ------------------------------------------------------------------------------------------------
 */

use leptos::prelude::*;
use std::collections::HashMap;
use uuid::Uuid;

use crate::presentation::molecules::table::*;
use crate::domain::job_context::*;
use crate::domain::runner_info::*;

#[component]
pub fn JobManagerView(
    jobs: Option<HashMap<Uuid, JobInfo>>,
    runners: Option<HashMap<Uuid, RunnerInfo>>,
) -> impl IntoView {
    view! {
        {move || {
            match &jobs {
                Some(jobs) => {
                    let rows = jobs.iter().map(|(_, job)| {
                        vec![
                            job.name.clone(),
                            job.status.to_string(),
                            job.priority.to_string(),
                            match &runners {
                                Some(list) => list.get(&job.runner_id.unwrap())
                                .unwrap_or(&RunnerInfo::new().name("Not Assigned")).name.clone(),
                                None => "Not Assigned".to_string(),
                            },
                            job.id.to_string()
                        ]
                    }).collect::<Vec<Vec<String>>>();

                    view! {
                        <Table table={TableStruct {
                            name: "Job List".to_string(),
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
                                        name: "Priority".to_string(),
                                        data_type: CellType::Int
                                    },
                                    ColumnDefinition {
                                        name: "Runner".to_string(),
                                        data_type: CellType::Text
                                    },
                                    ColumnDefinition {
                                        name: "Job ID".to_string(),
                                        data_type: CellType::Text
                                    }
                                ],
                                rows
                            }
                        }}/>
                    }.into_any()
                },
                None => view! { <div>"Loading Jobs..."</div> }.into_any()
            }
        }}
    }
}
