use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "lowercase")]
pub enum BlockSourceContext {
    Chat,
    Whisper,
}

impl BlockSourceContext {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Chat => "chat",
            Self::Whisper => "whisper",
        }
    }
}

impl AsRef<str> for BlockSourceContext {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl From<BlockSourceContext> for String {
    fn from(value: BlockSourceContext) -> Self {
        match value {
            BlockSourceContext::Chat => "chat".to_string(),
            BlockSourceContext::Whisper => "whisper".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "lowercase")]
pub enum BlockReason {
    Harassment,
    Spam,
    Other,
}

impl BlockReason {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Harassment => "harassment",
            Self::Spam => "spam",
            Self::Other => "other",
        }
    }
}

impl AsRef<str> for BlockReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl From<BlockReason> for String {
    fn from(value: BlockReason) -> String {
        match value {
            BlockReason::Harassment => "harassment".to_string(),
            BlockReason::Spam => "spam".to_string(),
            BlockReason::Other => "other".to_string(),
        }
    }
}
