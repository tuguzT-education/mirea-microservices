//! Data model of the microservice.

use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Blog data of the microservice.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Queryable)]
pub struct Blog {
    /// Identifier of the blog.
    pub blog_id: Uuid,
    /// Identifier of the user which owns the blog.
    pub user_id: Uuid,
    /// Name of the blog.
    pub name: String,
}

/// Blog data used to create blog.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct CreateBlog {
    /// Identifier of the user which owns the blog.
    pub user_id: Uuid,
    /// Name of the blog.
    pub name: String,
}

/// Blog data used to update blog.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct UpdateBlog {
    /// Identifier of the user which owns the blog.
    pub user_id: Uuid,
    /// Name of the blog.
    pub name: String,
}
