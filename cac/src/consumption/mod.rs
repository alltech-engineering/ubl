use serde::{Deserialize, Serialize};


include!("report.rs");
include!("line.rs");
include!("report_reference.rs");
include!("correction.rs");
include!("history.rs");
include!("average.rs");
include!("point.rs");

#[derive(Debug, Deserialize, Serialize)]
pub struct Consumption {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
    #[serde(default, rename = "UtilityStatementTypeCode")]
    pub utility_statement_type_code: Option<cct::Code>,
    #[serde(default, rename = "MainPeriod")]
    pub main_period: Option<crate::Period>,
    #[serde(default, rename = "AllowanceCharge")]
    pub allowance_charge: Vec<crate::AllowanceCharge>,
    #[serde(default, rename = "TaxTotal")]
    pub tax_total: Vec<crate::TaxTotal>,
    #[serde(default, rename = "EnergyWaterSupply")]
    pub energy_water_supply: Option<crate::EnergyWaterSupply>,
    #[serde(default, rename = "TelecommunicationsSupply")]
    pub telecommunications_supply: Option<crate::TelecommunicationsSupply>,
    #[serde(rename = "LegalMonetaryTotal")]
    pub legal_monetary_total: crate::MonetaryTotal,
}
