#[derive(Debug, Deserialize, Serialize)]
pub struct ExchangeRate {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
    #[serde(rename = "SourceCurrencyCode")]
    pub source_currency_code: cct::Code,
    #[serde(default, rename = "SourceCurrencyBaseRate")]
    pub source_currency_base_rate: Option<cct::Numeric>,
    #[serde(rename = "TargetCurrencyCode")]
    pub target_currency_code: cct::Code,
    #[serde(default, rename = "TargetCurrencyBaseRate")]
    pub target_currency_base_rate: Option<cct::Numeric>,
    #[serde(default, rename = "ExchangeMarketID")]
    pub exchange_market_id: Option<cct::Identifier>,
    #[serde(default, rename = "CalculationRate")]
    pub calculation_rate: Option<cct::Numeric>,
    #[serde(default, rename = "MathematicOperatorCode")]
    pub mathematic_operator_code: Option<cct::Code>,
    #[serde(default, rename = "Date")]
    pub date: Option<udt::DateTime>,
    #[serde(default, rename = "ForeignExchangeContract")]
    pub foreign_exchange_contract: Option<Contract>,
}
