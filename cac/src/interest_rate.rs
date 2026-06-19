#[derive(Debug, Deserialize, Serialize)]
pub struct InterestRate {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
    #[serde(rename = "InterestRatePercent")]
    pub interest_rate_percent: cct::Numeric,
    #[serde(default, rename = "TimeBasisCode")]
    pub time_basis_code: Option<cct::Code>,
    #[serde(default, rename = "CalculationMethodCode")]
    pub calculation_method_code: Option<cct::Code>,
}
