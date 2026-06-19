#[derive(Debug, Deserialize, Serialize)]
pub struct EnergyTaxReport {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "TaxEnergyAmount")]
    pub tax_energy_amount: Option<cct::Amount>,
    #[serde(default, rename = "TaxEnergyOnAccountAmount")]
    pub tax_energy_on_account_amount: Option<cct::Amount>,
    #[serde(default, rename = "TaxEnergyBalanceAmount")]
    pub tax_energy_balance_amount: Option<cct::Amount>,
    #[serde(rename = "TaxScheme")]
    pub tax_scheme: crate::TaxScheme,
}
