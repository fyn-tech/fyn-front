use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Clone, Debug, Default)]
pub enum RunnerState {
    Idle,
    Busy,
    Offline,
    Unregistered,
    #[default]
    Unknown,
}

#[derive(Clone, Debug, Default)]
pub struct RunnerInfo {
    pub id: Uuid,
    pub state: RunnerState,
    pub created_at: DateTime<Utc>,
    pub last_contact: Option<DateTime<Utc>>,
}

impl RunnerInfo {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn new_complete(
        id: Uuid,
        state: RunnerState,
        created_at: DateTime<Utc>,
        last_contact: Option<DateTime<Utc>>,
    ) -> Self {
        Self {
            id,
            state,
            created_at,
            last_contact,
        }
    }
}
