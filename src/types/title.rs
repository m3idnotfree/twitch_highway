use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Title {
    pub title: String,
}

impl Title {
    pub fn new<T: Into<String>>(title: T) -> Self {
        Self {
            title: title.into(),
        }
    }
}
