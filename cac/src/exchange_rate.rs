#[derive(Debug, Deserialize, Serialize)]
/// A class to define an exchange rate.
///
/// UBL Dictionary Entry Name: `Exchange Rate. Details`
///
/// Generated from XSD type `ExchangeRateType`.
pub struct ExchangeRate {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// The reference currency for this exchange rate; the currency from which the exchange is being made.
    #[serde(rename = "SourceCurrencyCode")]
    pub source_currency_code: cct::Code,
/// In the case of a source currency with denominations of small value, the unit base.
    #[serde(default, rename = "SourceCurrencyBaseRate")]
    pub source_currency_base_rate: Option<cct::Numeric>,
/// The target currency for this exchange rate; the currency to which the exchange is being made.
    #[serde(rename = "TargetCurrencyCode")]
    pub target_currency_code: cct::Code,
/// In the case of a target currency with denominations of small value, the unit base.
    #[serde(default, rename = "TargetCurrencyBaseRate")]
    pub target_currency_base_rate: Option<cct::Numeric>,
/// An identifier for the currency exchange market used as the source of this exchange rate.
    #[serde(default, rename = "ExchangeMarketID")]
    pub exchange_market_id: Option<cct::Identifier>,
/// The factor applied to the source currency to calculate the target currency.
    #[serde(default, rename = "CalculationRate")]
    pub calculation_rate: Option<cct::Numeric>,
/// A code signifying whether the calculation rate is a multiplier or a divisor.
    #[serde(default, rename = "MathematicOperatorCode")]
    pub mathematic_operator_code: Option<cct::Code>,
/// The date on which the exchange rate was established.
    #[serde(default, rename = "Date")]
    pub date: Option<udt::DateTime>,
/// A contract for foreign exchange.
    #[serde(default, rename = "ForeignExchangeContract")]
    pub foreign_exchange_contract: Option<Contract>,
}
