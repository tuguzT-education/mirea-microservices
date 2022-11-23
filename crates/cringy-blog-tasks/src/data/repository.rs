//! Repository definitions of the microservice.

use std::sync::Arc;

use async_trait::async_trait;
use derive_more::{Display, Error};
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::data::model::{CreateTask, Task, TaskCompletionState};

use super::model::UpdateTask;

/// Repository with task data of the microservice.
#[async_trait]
pub trait TaskRepository {
    /// Get all tasks.
    async fn get_all(&self) -> TaskRepoResult<Vec<Task>>;

    /// Find one task by its identifier.
    async fn get_one(&self, id: Uuid) -> TaskRepoResult<Task>;

    /// Create one task from the provided data.
    async fn create_one(&self, create: CreateTask) -> TaskRepoResult<Task>;

    /// Update one task which is found by provided task identifier.
    async fn update_one(&self, id: Uuid, update: UpdateTask) -> TaskRepoResult<Task>;

    /// Delete one task by its identifier.
    async fn delete_one(&self, id: Uuid) -> TaskRepoResult<Task>;
}

/// Task repository in-memory implementation.
#[derive(Debug, Default)]
pub struct InMemoryTaskRepository(Arc<RwLock<Vec<Task>>>);

#[async_trait]
impl TaskRepository for InMemoryTaskRepository {
    async fn get_all(&self) -> TaskRepoResult<Vec<Task>> {
        let data = self.0.read().await;
        Ok(data.clone())
    }

    async fn get_one(&self, id: Uuid) -> TaskRepoResult<Task> {
        let data = self.0.read().await;
        let Some(task) = data.iter().find(|task| task.task_id == id) else {
            return Err(TaskRepoError::NoTaskById);
        };
        Ok(task.clone())
    }

    async fn create_one(&self, create: CreateTask) -> TaskRepoResult<Task> {
        let mut data = self.0.write().await;
        let task = Task {
            task_id: Uuid::new_v4(),
            blog_id: create.blog_id,
            name: create.name,
            deadline: create.deadline,
            completion: TaskCompletionState::NotCompleted,
        };
        data.push(task.clone());
        Ok(task)
    }

    async fn update_one(&self, id: Uuid, update: UpdateTask) -> TaskRepoResult<Task> {
        let mut data = self.0.write().await;
        let Some(task) = data.iter_mut().find(|task| task.task_id == id) else {
            return Err(TaskRepoError::NoTaskById);
        };
        task.blog_id = update.blog_id;
        task.name = update.name;
        task.deadline = update.deadline;
        task.completion = update.completion;
        Ok(task.clone())
    }

    async fn delete_one(&self, id: Uuid) -> TaskRepoResult<Task> {
        let mut data = self.0.write().await;
        let Some(idx) = data.iter_mut().position(|task| task.task_id == id) else {
            return Err(TaskRepoError::NoTaskById);
        };
        let task = data.swap_remove(idx);
        Ok(task)
    }
}

/// Shared task repository accessed dynamically (as trait object).
pub type DynTaskRepository = Arc<dyn TaskRepository + Send + Sync>;

/// Task repository result type.
pub type TaskRepoResult<T> = Result<T, TaskRepoError>;

/// Error type returned on task repository error.
#[derive(Debug, Display, Error)]
pub enum TaskRepoError {
    /// Task already exists by identifier.
    #[display(fmt = "task already exists by id")]
    ExistsById,
    /// No task found by identifier.
    #[display(fmt = "no task by id")]
    NoTaskById,
}
