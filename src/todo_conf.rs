use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TodoConf {
    pub owner: String,
    pub conf_path: String,
    pub list_path: String,
}
