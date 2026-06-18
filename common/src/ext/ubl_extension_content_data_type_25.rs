use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize, Serialize)]
pub struct ExtensionContentType {
    #[serde(rename = "any125")]
    pub any: String,
}
