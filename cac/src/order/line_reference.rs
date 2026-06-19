#[derive(Debug, Deserialize, Serialize)]
pub struct OrderLineReference {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
    #[serde(rename = "LineID")]
    pub line_id: cct::Identifier,
    #[serde(default, rename = "SalesOrderLineID")]
    pub sales_order_line_id: Option<cct::Identifier>,
    #[serde(default, rename = "UUID")]
    pub uuid: Option<cct::Identifier>,
    #[serde(default, rename = "LineStatusCode")]
    pub line_status_code: Option<cct::Code>,
    #[serde(default, rename = "OrderReference")]
    pub order_reference: Option<OrderReference>,
}
