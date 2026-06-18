#[derive(Debug, Deserialize, Serialize)]
pub struct DeliveryTerms {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "SpecialTerms")]
    pub special_terms: Vec<super::cct::TextType>,
    #[serde(default, rename = "LossRiskResponsibilityCode")]
    pub loss_risk_responsibility_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "LossRisk")]
    pub loss_risk: Vec<super::cct::TextType>,
    #[serde(default, rename = "Amount")]
    pub amount: Option<super::cct::AmountType>,
    #[serde(default, rename = "DeliveryLocation")]
    pub delivery_location: Option<Location>,
    #[serde(default, rename = "AllowanceCharge")]
    pub allowance_charge: Option<AllowanceCharge>,
}
