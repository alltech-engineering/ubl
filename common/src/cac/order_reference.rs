#[derive(Debug, Deserialize, Serialize)]
pub struct OrderReference {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(rename = "ID")]
    pub id: super::cct::IdentifierType,
    #[serde(default, rename = "SalesOrderID")]
    pub sales_order_id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "CopyIndicator")]
    pub copy_indicator: Option<super::udt::IndicatorType>,
    #[serde(default, rename = "UUID")]
    pub uuid: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "IssueDate")]
    pub issue_date: Option<super::udt::DateTimeType>,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: Option<super::udt::DateTimeType>,
    #[serde(default, rename = "CustomerReference")]
    pub customer_reference: Option<super::cct::TextType>,
    #[serde(default, rename = "OrderTypeCode")]
    pub order_type_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "DocumentReference")]
    pub document_reference: Option<DocumentReference>,
}
