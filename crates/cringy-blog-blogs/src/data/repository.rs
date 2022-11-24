//! Repository definitions of the microservice.

use std::sync::Arc;

use async_trait::async_trait;
use derive_more::{Display, Error};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use uuid::Uuid;

use crate::data::model::{Blog, CreateBlog, UpdateBlog};

/// Repository with blog data of the microservice.
#[async_trait]
pub trait BlogRepository {
    /// Get all blogs.
    async fn get_all(&self) -> BlogRepoResult<Vec<Blog>>;

    /// Find one blog by its identifier.
    async fn get_one(&self, id: Uuid) -> BlogRepoResult<Blog>;

    /// Create one blog from the provided data.
    async fn create_one(&self, create: CreateBlog) -> BlogRepoResult<Blog>;

    /// Update one blog which is found by provided blog identifier.
    async fn update_one(&self, id: Uuid, update: UpdateBlog) -> BlogRepoResult<Blog>;

    /// Delete one blog by its identifier.
    async fn delete_one(&self, id: Uuid) -> BlogRepoResult<Blog>;
}

/// Blog repository in-memory implementation.
#[derive(Debug)]
pub struct LocalBlogRepository {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl LocalBlogRepository {
    /// Creates new local blog repository.
    pub fn new(pool: Pool<ConnectionManager<PgConnection>>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl BlogRepository for LocalBlogRepository {
    async fn get_all(&self) -> BlogRepoResult<Vec<Blog>> {
        use crate::schema::blogs::dsl::*;

        let conn = &mut self.pool.get().unwrap();
        let data = blogs.load(conn).unwrap();
        Ok(data)
    }

    async fn get_one(&self, id: Uuid) -> BlogRepoResult<Blog> {
        use crate::schema::blogs::dsl::*;

        let conn = &mut self.pool.get().unwrap();
        let Ok(task) = blogs.filter(blog_id.eq(id)).first(conn) else {
            return Err(BlogRepoError::NoBlogById);
        };
        Ok(task)
    }

    async fn create_one(&self, create: CreateBlog) -> BlogRepoResult<Blog> {
        use crate::schema::blogs::dsl::*;

        #[derive(Debug, Insertable)]
        #[diesel(table_name = crate::schema::blogs)]
        struct NewBlog {
            blog_id: Uuid,
            user_id: Uuid,
            name: String,
        }

        let conn = &mut self.pool.get().unwrap();
        let task = NewBlog {
            blog_id: Uuid::new_v4(),
            user_id: create.user_id,
            name: create.name,
        };
        let task = diesel::insert_into(blogs)
            .values(task)
            .get_result(conn)
            .unwrap();
        Ok(task)
    }

    async fn update_one(&self, id: Uuid, update: UpdateBlog) -> BlogRepoResult<Blog> {
        use crate::schema::blogs::dsl::*;

        let conn = &mut self.pool.get().unwrap();
        let task = diesel::update(blogs.find(id))
            .set((user_id.eq(update.user_id), name.eq(update.name)))
            .get_result(conn)
            .unwrap();
        Ok(task)
    }

    async fn delete_one(&self, id: Uuid) -> BlogRepoResult<Blog> {
        use crate::schema::blogs::dsl::*;

        let conn = &mut self.pool.get().unwrap();
        let Ok(task) = blogs.filter(blog_id.eq(id)).first(conn) else {
            return Err(BlogRepoError::NoBlogById);
        };
        diesel::delete(blogs.filter(blog_id.eq(id)))
            .execute(conn)
            .unwrap();
        Ok(task)
    }
}

/// Shared blog repository accessed dynamically (as trait object).
pub type DynBlogRepository = Arc<dyn BlogRepository + Send + Sync>;

/// Blog repository result type.
pub type BlogRepoResult<T> = Result<T, BlogRepoError>;

/// Error type returned on blog repository error.
#[derive(Debug, Display, Error)]
pub enum BlogRepoError {
    /// Blog already exists by identifier.
    #[display(fmt = "blog already exists by id")]
    ExistsById,
    /// No blog found by identifier.
    #[display(fmt = "no blog by id")]
    NoBlogById,
}
