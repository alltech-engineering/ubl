#[derive(Debug, Deserialize, Serialize)]
pub struct OrderReference {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(rename = "ID")]
    pub id: cct::Identifier,
    #[serde(default, rename = "SalesOrderID")]
    pub sales_order_id: Option<cct::Identifier>,
    #[serde(default, rename = "CopyIndicator")]
    pub copy_indicator: Option<udt::Indicator>,
    #[serde(default, rename = "UUID")]
    pub uuid: Option<cct::Identifier>,
    #[serde(default, rename = "IssueDate")]
    pub issue_date: Option<udt::DateTime>,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: Option<udt::DateTime>,
    #[serde(default, rename = "CustomerReference")]
    pub customer_reference: Option<cct::Text>,
    #[serde(default, rename = "OrderTypeCode")]
    pub order_type_code: Option<cct::Code>,
    #[serde(default, rename = "DocumentReference")]
    pub document_reference: Option<crate::DocumentReference>,
}
