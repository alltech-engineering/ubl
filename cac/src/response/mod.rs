use serde::{Deserialize, Serialize};


include!("value.rs");

#[derive(Debug, Deserialize, Serialize)]
pub struct Response {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
    #[serde(default, rename = "ReferenceID")]
    pub reference_id: Option<cct::Identifier>,
    #[serde(default, rename = "ResponseCode")]
    pub response_code: Option<cct::Code>,
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
    #[serde(default, rename = "EffectiveDate")]
    pub effective_date: Option<udt::DateTime>,
    #[serde(default, rename = "EffectiveTime")]
    pub effective_time: Option<udt::DateTime>,
    #[serde(default, rename = "Status")]
    pub status: Vec<crate::Status>,
}
