//! Repository definitions of the microservice.

use std::sync::Arc;

use async_trait::async_trait;
use derive_more::{Display, Error};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
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
#[derive(Debug)]
pub struct LocalTaskRepository {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl LocalTaskRepository {
    /// Creates new local task repository.
    pub fn new(pool: Pool<ConnectionManager<PgConnection>>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TaskRepository for LocalTaskRepository {
    async fn get_all(&self) -> TaskRepoResult<Vec<Task>> {
        use crate::schema::tasks::dsl::*;

        let conn = &mut self.pool.get().unwrap();
        let data = tasks.load(conn).unwrap();
        Ok(data)
    }

    async fn get_one(&self, id: Uuid) -> TaskRepoResult<Task> {
        use crate::schema::tasks::dsl::*;

        let conn = &mut self.pool.get().unwrap();
        let Ok(task) = tasks.filter(task_id.eq(id)).first(conn) else {
            return Err(TaskRepoError::NoTaskById);
        };
        Ok(task)
    }

    async fn create_one(&self, create: CreateTask) -> TaskRepoResult<Task> {
        use crate::schema::tasks::dsl::*;
        use chrono::{DateTime, Utc};

        #[derive(Debug, Insertable)]
        #[diesel(table_name = crate::schema::tasks)]
        struct NewTask {
            task_id: Uuid,
            blog_id: Uuid,
            name: String,
            deadline: Option<DateTime<Utc>>,
            completion: TaskCompletionState,
        }

        let conn = &mut self.pool.get().unwrap();
        let task = NewTask {
            task_id: Uuid::new_v4(),
            blog_id: create.blog_id,
            name: create.name,
            deadline: create.deadline,
            completion: TaskCompletionState::NotCompleted,
        };
        let task = diesel::insert_into(tasks)
            .values(task)
            .get_result(conn)
            .unwrap();
        Ok(task)
    }

    async fn update_one(&self, id: Uuid, update: UpdateTask) -> TaskRepoResult<Task> {
        use crate::schema::tasks::dsl::*;

        let conn = &mut self.pool.get().unwrap();
        let task = diesel::update(tasks.find(id))
            .set((
                blog_id.eq(update.blog_id),
                name.eq(update.name),
                deadline.eq(update.deadline),
                completion.eq(update.completion),
            ))
            .get_result(conn)
            .unwrap();
        Ok(task)
    }

    async fn delete_one(&self, id: Uuid) -> TaskRepoResult<Task> {
        use crate::schema::tasks::dsl::*;

        let conn = &mut self.pool.get().unwrap();
        let Ok(task) = tasks.filter(task_id.eq(id)).first(conn) else {
            return Err(TaskRepoError::NoTaskById);
        };
        diesel::delete(tasks.filter(task_id.eq(id)))
            .execute(conn)
            .unwrap();
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
