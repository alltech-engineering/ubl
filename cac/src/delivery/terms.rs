#[derive(Debug, Deserialize, Serialize)]
pub struct DeliveryTerms {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
    #[serde(default, rename = "SpecialTerms")]
    pub special_terms: Vec<cct::Text>,
    #[serde(default, rename = "LossRiskResponsibilityCode")]
    pub loss_risk_responsibility_code: Option<cct::Code>,
    #[serde(default, rename = "LossRisk")]
    pub loss_risk: Vec<cct::Text>,
    #[serde(default, rename = "Amount")]
    pub amount: Option<cct::Amount>,
    #[serde(default, rename = "DeliveryLocation")]
    pub delivery_location: Option<crate::Location>,
    #[serde(default, rename = "AllowanceCharge")]
    pub allowance_charge: Option<crate::AllowanceCharge>,
}
