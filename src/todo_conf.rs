use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TodoConf {
    pub owner: String,
    pub list_path: String,
}
