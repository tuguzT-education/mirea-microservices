// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "task_completion_state"))]
    pub struct TaskCompletionState;
}

diesel::table! {
    blogs (blog_id) {
        blog_id -> Uuid,
        user_id -> Uuid,
        name -> Varchar,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::TaskCompletionState;

    tasks (task_id) {
        task_id -> Uuid,
        blog_id -> Uuid,
        name -> Varchar,
        deadline -> Nullable<Timestamptz>,
        completion -> TaskCompletionState,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    blogs,
    tasks,
);
