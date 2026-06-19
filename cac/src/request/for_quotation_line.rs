#[derive(Debug, Deserialize, Serialize)]
pub struct RequestForQuotationLine {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
    #[serde(default, rename = "UUID")]
    pub uuid: Option<cct::Identifier>,
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
    #[serde(default, rename = "OptionalLineItemIndicator")]
    pub optional_line_item_indicator: Option<udt::Indicator>,
    #[serde(default, rename = "PrivacyCode")]
    pub privacy_code: Option<cct::Code>,
    #[serde(default, rename = "SecurityClassificationCode")]
    pub security_classification_code: Option<cct::Code>,
    #[serde(default, rename = "DocumentReference")]
    pub document_reference: Vec<crate::DocumentReference>,
    #[serde(rename = "LineItem")]
    pub line_item: crate::LineItem,
}
