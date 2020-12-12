use rocket::{get, post};

use crate::types::Project;

#[get("/")]
fn root() -> String {
    "It woks!".to_string
}

#[get("/projects")]
pub fn get_projects() -> Vec<Project> {
    Vec![]
}

#[post("/projects")]
pub fn create_projects() -> Project {
    Vec![]
}