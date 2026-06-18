#[derive(Debug, Deserialize, Serialize)]
pub struct FinancialGuarantee {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(rename = "GuaranteeTypeCode")]
    pub guarantee_type_code: super::cct::CodeType,
    #[serde(default, rename = "Description")]
    pub description: Vec<super::cct::TextType>,
    #[serde(default, rename = "LiabilityAmount")]
    pub liability_amount: Option<super::cct::AmountType>,
    #[serde(default, rename = "AmountRate")]
    pub amount_rate: Option<super::cct::NumericType>,
    #[serde(default, rename = "ConstitutionPeriod")]
    pub constitution_period: Option<Period>,
}
