#[derive(Debug, Deserialize, Serialize)]
pub struct EnergyTaxReport {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
    #[serde(default, rename = "TaxEnergyAmount")]
    pub tax_energy_amount: Option<cct::Amount>,
    #[serde(default, rename = "TaxEnergyOnAccountAmount")]
    pub tax_energy_on_account_amount: Option<cct::Amount>,
    #[serde(default, rename = "TaxEnergyBalanceAmount")]
    pub tax_energy_balance_amount: Option<cct::Amount>,
    #[serde(rename = "TaxScheme")]
    pub tax_scheme: crate::TaxScheme,
}
