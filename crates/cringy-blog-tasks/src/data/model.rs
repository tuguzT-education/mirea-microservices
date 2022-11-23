//! Data model of the microservice.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Task data of the microservice.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Task {
    /// Identifier of the task.
    pub task_id: Uuid,
    /// Identifier of the blog which owns the task.
    pub blog_id: Uuid,
    /// Name of the task.
    pub name: String,
    /// Deadline of the task.
    pub deadline: Option<DateTime<Utc>>,
    /// Completion state of the task.
    pub completion: TaskCompletionState,
}

/// Task data used to create task.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct CreateTask {
    /// Identifier of the blog which owns the task.
    pub blog_id: Uuid,
    /// Name of the task.
    pub name: String,
    /// Deadline of the task.
    pub deadline: Option<DateTime<Utc>>,
}

/// Task data used to update task.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct UpdateTask {
    /// Identifier of the blog which owns the task.
    pub blog_id: Uuid,
    /// Name of the task.
    pub name: String,
    /// Deadline of the task.
    pub deadline: Option<DateTime<Utc>>,
    /// Completion state of the task.
    pub completion: TaskCompletionState,
}

/// Task completion state.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum TaskCompletionState {
    /// The task is completed.
    Completed,
    /// The task is not completed.
    NotCompleted,
}
