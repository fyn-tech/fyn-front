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

use std::fmt;

use serde_json::Value;
use std::path::PathBuf;
use uuid::Uuid;

// -------------------------------------------------------------------------------------------------
// Job Related Enums
// -------------------------------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum JobStatus {
    #[default]
    UploadingInputResources, // used as default -> does not trigger backend (i.e. is inert)
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

impl fmt::Display for JobStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JobStatus::UploadingInputResources => write!(f, "Uploading Input Resources"),
            JobStatus::Queued => write!(f, "Queued"),
            JobStatus::Preparing => write!(f, "Preparing"),
            JobStatus::FetchingResources => write!(f, "Fetching Resources"),
            JobStatus::Starting => write!(f, "Starting"),
            JobStatus::Running => write!(f, "Running"),
            JobStatus::Paused => write!(f, "Paused"),
            JobStatus::CleaningUp => write!(f, "Cleaning Up"),
            JobStatus::UploadingResults => write!(f, "Uploading Results"),
            JobStatus::Succeeded => write!(f, "Succeeded"),
            JobStatus::Failed => write!(f, "Failed"),
            JobStatus::FailedResourceError => write!(f, "Failed - Resource Error"),
            JobStatus::FailedTerminated => write!(f, "Failed - Terminated"),
            JobStatus::FailedTimeout => write!(f, "Failed - Timeout"),
            JobStatus::FailedRunnerException => write!(f, "Failed - Runner Exception"),
        }
    }
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

#[derive(Clone, Debug, Default)]
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

#[allow(dead_code)]
impl JobInfo {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn id(mut self, id: Uuid) -> Self {
        self.id = id;
        self
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    pub fn status(mut self, status: JobStatus) -> Self {
        self.status = status;
        self
    }

    pub fn application_id(mut self, application_id: Uuid) -> Self {
        self.application_id = application_id;
        self
    }

    pub fn priority(mut self, priority: i64) -> Self {
        self.priority = priority;
        self
    }

    pub fn runner_id(mut self, runner_id: Uuid) -> Self {
        self.runner_id = Some(runner_id);
        self
    }

    pub fn maybe_runner_id(mut self, runner_id: Option<Uuid>) -> Self {
        self.runner_id = runner_id;
        self
    }

    pub fn executable(mut self, executable: impl Into<String>) -> Self {
        self.executable = executable.into();
        self
    }

    pub fn command_line_args(mut self, command_line_args: &Value) -> Self {
        self.command_line_args = Some(command_line_args.clone());
        self
    }

    pub fn maybe_command_line_args(mut self, command_line_args: &Option<Value>) -> Self {
        self.command_line_args = command_line_args.clone();
        self
    }

    pub fn exit_code(mut self, exit_code: i64) -> Self {
        self.exit_code = Some(exit_code);
        self
    }

    pub fn maybe_exit_code(mut self, exit_code: Option<i64>) -> Self {
        self.exit_code = exit_code;
        self
    }

    pub fn resources(mut self, resources: &Vec<Uuid>) -> Self {
        self.resources = resources.clone();
        self
    }

    pub fn build(self) -> Result<Self, String> {
        validate_cl_args(&self.command_line_args)?;
        Ok(self)
    }

    pub fn set_id(&mut self, id: Uuid) -> Result<(), String> {
        if self.id != Uuid::nil() {
            return Err("Job Resource ID must not be nil".to_string());
        }
        self.id = id;
        Ok(())
    }
}

// -------------------------------------------------------------------------------------------------
// JobResource
// -------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct JobResource {
    pub id: Uuid,
    pub resource_type: ResourceType,
    pub file: String,
    pub description: String,
    pub original_file_path: PathBuf,
}

#[allow(dead_code)]
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
