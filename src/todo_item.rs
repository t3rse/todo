use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TodoItem {
    pub id: String,
    pub create: String,
    pub entry: String,
    pub context_info: Vec<String>,
    pub project_info: Vec<String>,
}
