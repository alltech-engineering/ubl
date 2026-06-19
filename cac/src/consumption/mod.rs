use serde::{Deserialize, Serialize};


include!("report.rs");
include!("line.rs");
include!("report_reference.rs");
include!("correction.rs");
include!("history.rs");
include!("average.rs");
include!("point.rs");

#[derive(Debug, Deserialize, Serialize)]
/// A class to describe the consumption of a utility.
///
/// UBL Dictionary Entry Name: `Consumption. Details`
///
/// Generated from XSD type `ConsumptionType`.
pub struct Consumption {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// A code identifying the type of the Utility Statement required for this consumption. Explains the
/// kind of utility the statement is about, e.g.. "gas", "electricity", "telephone"
    #[serde(default, rename = "UtilityStatementTypeCode")]
    pub utility_statement_type_code: Option<cct::Code>,
/// The period of consumption.
    #[serde(default, rename = "MainPeriod")]
    pub main_period: Option<crate::Period>,
/// An allowance or charges that may apply with this consumption.
    #[serde(default, rename = "AllowanceCharge")]
    pub allowance_charge: Vec<crate::AllowanceCharge>,
/// The total of taxes for each tax type covering the consumption.
    #[serde(default, rename = "TaxTotal")]
    pub tax_total: Vec<crate::TaxTotal>,
/// The details of any energy or water consumption.
    #[serde(default, rename = "EnergyWaterSupply")]
    pub energy_water_supply: Option<crate::EnergyWaterSupply>,
/// The details of any telecommunications consumption.
    #[serde(default, rename = "TelecommunicationsSupply")]
    pub telecommunications_supply: Option<crate::TelecommunicationsSupply>,
/// The total amount payable on this consumption, including any allowances, charges, or taxes.
    #[serde(rename = "LegalMonetaryTotal")]
    pub legal_monetary_total: crate::MonetaryTotal,
}
