#[derive(Debug, Deserialize, Serialize)]
/// A class to describe an interest rate applied to a monetary amount over a defined period.
///
/// UBL Dictionary Entry Name: `Interest Rate. Details`
///
/// Generated from XSD type `InterestRateType`.
pub struct InterestRate {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// The numeric value of the interest rate expressed as a percentage.
    #[serde(rename = "InterestRatePercent")]
    pub interest_rate_percent: cct::Numeric,
/// A code specifying the time basis to which the interest rate applies, such as per annum or per day.
    #[serde(default, rename = "TimeBasisCode")]
    pub time_basis_code: Option<cct::Code>,
/// A code specifying how the interest is calculated (e.g., simple, compount).
    #[serde(default, rename = "CalculationMethodCode")]
    pub calculation_method_code: Option<cct::Code>,
}
