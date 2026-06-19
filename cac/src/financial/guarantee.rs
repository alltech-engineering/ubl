#[derive(Debug, Deserialize, Serialize)]
pub struct FinancialGuarantee {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
    #[serde(rename = "GuaranteeTypeCode")]
    pub guarantee_type_code: cct::Code,
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
    #[serde(default, rename = "LiabilityAmount")]
    pub liability_amount: Option<cct::Amount>,
    #[serde(default, rename = "AmountRate")]
    pub amount_rate: Option<cct::Numeric>,
    #[serde(default, rename = "ConstitutionPeriod")]
    pub constitution_period: Option<crate::Period>,
}
