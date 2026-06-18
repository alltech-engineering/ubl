#[derive(Debug, Deserialize, Serialize)]
pub struct ConsumptionLine {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(rename = "ID")]
    pub id: super::cct::IdentifierType,
    #[serde(default, rename = "ParentDocumentLineReferenceID")]
    pub parent_document_line_reference_id: Option<super::cct::IdentifierType>,
    #[serde(rename = "InvoicedQuantity")]
    pub invoiced_quantity: super::cct::QuantityType,
    #[serde(rename = "LineExtensionAmount")]
    pub line_extension_amount: super::cct::AmountType,
    #[serde(default, rename = "TaxInclusiveLineExtensionAmount")]
    pub tax_inclusive_line_extension_amount: Option<super::cct::AmountType>,
    #[serde(default, rename = "Period")]
    pub period: Option<Period>,
    #[serde(default, rename = "Delivery")]
    pub delivery: Vec<Delivery>,
    #[serde(default, rename = "AllowanceCharge")]
    pub allowance_charge: Vec<AllowanceCharge>,
    #[serde(default, rename = "TaxTotal")]
    pub tax_total: Vec<TaxTotal>,
    #[serde(rename = "UtilityItem")]
    pub utility_item: UtilityItem,
    #[serde(default, rename = "Price")]
    pub price: Option<Price>,
    #[serde(default, rename = "UnstructuredPrice")]
    pub unstructured_price: Option<UnstructuredPrice>,
}
