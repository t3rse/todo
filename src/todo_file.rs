use serde::{Deserialize, Serialize};
use crate::todo_item::TodoItem;

#[derive(Serialize, Deserialize, Debug)]
pub struct TodoFile {
    pub items: Vec<TodoItem>
}