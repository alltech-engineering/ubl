#[derive(Debug, Deserialize, Serialize)]
pub struct BillingReferenceLine {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(rename = "ID")]
    pub id: cct::Identifier,
    #[serde(default, rename = "Amount")]
    pub amount: Option<cct::Amount>,
    #[serde(default, rename = "AllowanceCharge")]
    pub allowance_charge: Vec<crate::AllowanceCharge>,
}
