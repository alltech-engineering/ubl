#[derive(Debug, Deserialize, Serialize)]
pub struct ExchangeRate {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(rename = "SourceCurrencyCode")]
    pub source_currency_code: super::cct::CodeType,
    #[serde(default, rename = "SourceCurrencyBaseRate")]
    pub source_currency_base_rate: Option<super::cct::NumericType>,
    #[serde(rename = "TargetCurrencyCode")]
    pub target_currency_code: super::cct::CodeType,
    #[serde(default, rename = "TargetCurrencyBaseRate")]
    pub target_currency_base_rate: Option<super::cct::NumericType>,
    #[serde(default, rename = "ExchangeMarketID")]
    pub exchange_market_id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "CalculationRate")]
    pub calculation_rate: Option<super::cct::NumericType>,
    #[serde(default, rename = "MathematicOperatorCode")]
    pub mathematic_operator_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "Date")]
    pub date: Option<super::udt::DateTimeType>,
    #[serde(default, rename = "ForeignExchangeContract")]
    pub foreign_exchange_contract: Option<Contract>,
}
