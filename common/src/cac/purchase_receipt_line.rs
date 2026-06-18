#[derive(Debug, Deserialize, Serialize)]
pub struct PurchaseReceiptLine {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(rename = "ID")]
    pub id: super::cct::IdentifierType,
    #[serde(default, rename = "UUID")]
    pub uuid: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "Note")]
    pub note: Vec<super::cct::TextType>,
    #[serde(default, rename = "Quantity")]
    pub quantity: Option<super::cct::QuantityType>,
    #[serde(default, rename = "LineExtensionAmount")]
    pub line_extension_amount: Option<super::cct::AmountType>,
    #[serde(default, rename = "TaxInclusiveLineExtensionAmount")]
    pub tax_inclusive_line_extension_amount: Option<super::cct::AmountType>,
    #[serde(default, rename = "PurchaseLinePeriod")]
    pub purchase_line_period: Option<Period>,
    #[serde(default, rename = "PurchaseReference")]
    pub purchase_reference: Option<PurchaseReference>,
    #[serde(default, rename = "AllowanceCharge")]
    pub allowance_charge: Vec<AllowanceCharge>,
    #[serde(default, rename = "TaxTotal")]
    pub tax_total: Vec<TaxTotal>,
    #[serde(rename = "Item")]
    pub item: Item,
    #[serde(default, rename = "Price")]
    pub price: Option<Price>,
}
