#[derive(Debug, Deserialize, Serialize)]
pub struct ConsumptionLine {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
    #[serde(rename = "ID")]
    pub id: cct::Identifier,
    #[serde(default, rename = "ParentDocumentLineReferenceID")]
    pub parent_document_line_reference_id: Option<cct::Identifier>,
    #[serde(rename = "InvoicedQuantity")]
    pub invoiced_quantity: cct::Quantity,
    #[serde(rename = "LineExtensionAmount")]
    pub line_extension_amount: cct::Amount,
    #[serde(default, rename = "TaxInclusiveLineExtensionAmount")]
    pub tax_inclusive_line_extension_amount: Option<cct::Amount>,
    #[serde(default, rename = "Period")]
    pub period: Option<crate::Period>,
    #[serde(default, rename = "Delivery")]
    pub delivery: Vec<crate::Delivery>,
    #[serde(default, rename = "AllowanceCharge")]
    pub allowance_charge: Vec<crate::AllowanceCharge>,
    #[serde(default, rename = "TaxTotal")]
    pub tax_total: Vec<crate::TaxTotal>,
    #[serde(rename = "UtilityItem")]
    pub utility_item: crate::UtilityItem,
    #[serde(default, rename = "Price")]
    pub price: Option<crate::Price>,
    #[serde(default, rename = "UnstructuredPrice")]
    pub unstructured_price: Option<crate::UnstructuredPrice>,
}
