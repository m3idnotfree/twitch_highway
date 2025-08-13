use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RequestBody<T, K> {
    #[serde(flatten)]
    required: T,
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    opt: Option<K>,
}

impl<T, K> RequestBody<T, K>
where
    T: Serialize,
    K: Serialize,
{
    pub fn new(required: T, opts: Option<K>) -> Self {
        Self {
            required,
            opt: opts,
        }
    }

    pub fn into_json(&self) -> Option<String> {
        Some(serde_json::to_string(self).unwrap())
    }
}
