#[derive(Debug, Deserialize, Serialize)]
pub struct Consumption {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "UtilityStatementTypeCode")]
    pub utility_statement_type_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "MainPeriod")]
    pub main_period: Option<Period>,
    #[serde(default, rename = "AllowanceCharge")]
    pub allowance_charge: Vec<AllowanceCharge>,
    #[serde(default, rename = "TaxTotal")]
    pub tax_total: Vec<TaxTotal>,
    #[serde(default, rename = "EnergyWaterSupply")]
    pub energy_water_supply: Option<EnergyWaterSupply>,
    #[serde(default, rename = "TelecommunicationsSupply")]
    pub telecommunications_supply: Option<TelecommunicationsSupply>,
    #[serde(rename = "LegalMonetaryTotal")]
    pub legal_monetary_total: MonetaryTotal,
}
