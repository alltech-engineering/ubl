#[derive(Debug, Deserialize, Serialize)]
pub struct LineResponse {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(rename = "LineReference")]
    pub line_reference: LineReference,
    #[serde(default, rename = "Response")]
    pub response: Vec<Response>,
}
