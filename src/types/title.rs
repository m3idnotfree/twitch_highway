use serde::{Deserialize, Serialize};

request_struct!(
    #[derive(Debug, Serialize, Deserialize)]
    Title {
        required {
            title: String
        }
    }
);
