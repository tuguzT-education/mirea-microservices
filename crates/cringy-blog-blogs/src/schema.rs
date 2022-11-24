// @generated automatically by Diesel CLI.

diesel::table! {
    blogs (blog_id) {
        blog_id -> Uuid,
        user_id -> Uuid,
        name -> Varchar,
    }
}
