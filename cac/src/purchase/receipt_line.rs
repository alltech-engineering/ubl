#[derive(Debug, Deserialize, Serialize)]
pub struct PurchaseReceiptLine {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(rename = "ID")]
    pub id: cct::Identifier,
    #[serde(default, rename = "UUID")]
    pub uuid: Option<cct::Identifier>,
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
    #[serde(default, rename = "Quantity")]
    pub quantity: Option<cct::Quantity>,
    #[serde(default, rename = "LineExtensionAmount")]
    pub line_extension_amount: Option<cct::Amount>,
    #[serde(default, rename = "TaxInclusiveLineExtensionAmount")]
    pub tax_inclusive_line_extension_amount: Option<cct::Amount>,
    #[serde(default, rename = "PurchaseLinePeriod")]
    pub purchase_line_period: Option<crate::Period>,
    #[serde(default, rename = "PurchaseReference")]
    pub purchase_reference: Option<PurchaseReference>,
    #[serde(default, rename = "AllowanceCharge")]
    pub allowance_charge: Vec<crate::AllowanceCharge>,
    #[serde(default, rename = "TaxTotal")]
    pub tax_total: Vec<crate::TaxTotal>,
    #[serde(rename = "Item")]
    pub item: crate::Item,
    #[serde(default, rename = "Price")]
    pub price: Option<crate::Price>,
}
