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
 * filename: job_info.rs
 * description: Domain entity representing the job context
 * ------------------------------------------------------------------------------------------------
 */
use std::path::PathBuf;
use std::{default, f32::consts::E};

use chrono::{DateTime, Utc};
use leptos::error::throw;
use serde_json::Value;
use uuid::Uuid;

// -------------------------------------------------------------------------------------------------
// Job Related Enums
// -------------------------------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum JobStatus {
    UploadingInputResources,
    Queued,
    Preparing,
    FetchingResources,
    Starting,
    Running,
    Paused,
    CleaningUp,
    UploadingResults,
    Succeeded,
    Failed,
    FailedResourceError,
    FailedTerminated,
    FailedTimeout,
    FailedRunnerException,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ResourceType {
    Input,
    Output,
    Config,
    Log,
    Temp,
    Result,
}

// -------------------------------------------------------------------------------------------------
// JobInfo
// -------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct JobInfo {
    pub id: Uuid,
    pub name: String,
    pub status: JobStatus,
    pub application_id: Uuid,
    pub priority: i64,
    pub runner_id: Option<Uuid>,
    pub executable: String,
    pub command_line_args: Option<Value>, // needs to be a list of values as the root node
    pub exit_code: Option<i64>,
    pub resources: Vec<Uuid>,
}

impl JobInfo {
    pub fn new(
        id: Uuid,
        name: String,
        status: JobStatus,
        app_id: Uuid,
        runner_id: Option<Uuid>,
        priority: i64,
        executable: String,
        cl_args: Option<Value>,
        exit_code: Option<i64>,
        resources: Vec<Uuid>,
    ) -> Result<JobInfo, String> {
        match validate_cl_args(&cl_args) {
            Ok(_) => Ok(Self {
                id: id,
                name: name,
                status: status,
                application_id: app_id,
                priority: priority,
                runner_id: runner_id,
                executable: executable,
                command_line_args: cl_args,
                exit_code: exit_code,
                resources: resources,
            }),
            Err(e) => Err(e),
        }
    }

    pub fn new_request(
        name: String,
        app_id: Uuid,
        runner_id: Option<Uuid>,
        priority: i64,
        executable: String,
        cl_args: Option<Value>,
    ) -> Result<JobInfo, String> {
        match validate_cl_args(&cl_args) {
            Ok(_) => Ok(Self {
                id: Uuid::nil(),
                name: name,
                status: JobStatus::UploadingInputResources,
                application_id: app_id,
                priority: priority,
                runner_id: runner_id,
                executable: executable,
                command_line_args: cl_args,
                exit_code: None,
                resources: vec![],
            }),
            Err(e) => Err(e),
        }
    }

    pub fn set_id(&mut self, id: Uuid) -> Result<(), String> {
        if self.id != Uuid::nil() {
            return Err("Job Resource ID must be nil".to_string());
        }
        self.id = id;
        Ok(())
    }
}

// -------------------------------------------------------------------------------------------------
// JobResource
// -------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct JobResource {
    pub id: Uuid,
    pub resource_type: ResourceType,
    pub file: String,
    pub description: String,
    pub original_file_path: PathBuf,
}

impl JobResource {
    pub fn new_resource(
        resource_type: ResourceType,
        file: String,
        description: String,
        file_path: PathBuf,
    ) -> JobResource {
        Self {
            id: Uuid::nil(),
            resource_type: resource_type,
            file: file,
            description: description,
            original_file_path: file_path,
        }
    }

    pub fn set_id(&mut self, id: Uuid) -> Result<(), String> {
        if self.id != Uuid::nil() {
            return Err("Job Resource ID must be nil".to_string());
        }
        self.id = id;
        Ok(())
    }
}

// -------------------------------------------------------------------------------------------------
// Support Functions
// -------------------------------------------------------------------------------------------------

/// Validates that a serde_json::Value is a list of strings only
fn validate_cl_args(option_value: &Option<Value>) -> Result<(), String> {
    match option_value {
        Some(value) => match value.as_array() {
            Some(arr) => {
                for item in arr {
                    if !item.is_string() {
                        return Err("Command line args must contain only strings".to_string());
                    }
                }
                Ok(())
            }
            None => Err("Command line args must be a list".to_string()),
        },
        None => Ok(()),
    }
}
