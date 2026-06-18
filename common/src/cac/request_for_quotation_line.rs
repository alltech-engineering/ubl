#[derive(Debug, Deserialize, Serialize)]
pub struct RequestForQuotationLine {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "UUID")]
    pub uuid: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "Note")]
    pub note: Vec<super::cct::TextType>,
    #[serde(default, rename = "OptionalLineItemIndicator")]
    pub optional_line_item_indicator: Option<super::udt::IndicatorType>,
    #[serde(default, rename = "PrivacyCode")]
    pub privacy_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "SecurityClassificationCode")]
    pub security_classification_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "DocumentReference")]
    pub document_reference: Vec<DocumentReference>,
    #[serde(rename = "LineItem")]
    pub line_item: LineItem,
}
