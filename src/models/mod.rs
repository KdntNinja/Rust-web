use serde::Serialize;

#[derive(Serialize)]
pub struct TemplateContext {
    pub title: String,
    pub name: String,
}
