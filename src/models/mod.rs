use serde::Serialize;

pub mod order;
pub mod user;

#[derive(Serialize)]
pub struct TemplateContext {
    pub title: String,
    pub name: String,
}
