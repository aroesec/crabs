//Need to use passwors crate to hash/salt pw's for db storage
mod task.models;

pub struct User {
    pub guid: String,
    pub username: String,
    pub password: String,
    pub tasks : task.models,
}
