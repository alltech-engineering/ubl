#[derive(Debug, Deserialize, Serialize)]
pub struct Response {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "ReferenceID")]
    pub reference_id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "ResponseCode")]
    pub response_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "Description")]
    pub description: Vec<super::cct::TextType>,
    #[serde(default, rename = "EffectiveDate")]
    pub effective_date: Option<super::udt::DateTimeType>,
    #[serde(default, rename = "EffectiveTime")]
    pub effective_time: Option<super::udt::DateTimeType>,
    #[serde(default, rename = "Status")]
    pub status: Vec<Status>,
}
