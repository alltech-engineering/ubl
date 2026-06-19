#[derive(Debug, Deserialize, Serialize)]
/// A class to describe energy taxes.
///
/// UBL Dictionary Entry Name: `Energy Tax Report. Details`
///
/// Generated from XSD type `EnergyTaxReportType`.
pub struct EnergyTaxReport {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// The monetary amount of taxes (and duties).
    #[serde(default, rename = "TaxEnergyAmount")]
    pub tax_energy_amount: Option<cct::Amount>,
/// The monetary amount of taxes (and duties) paid on account.
    #[serde(default, rename = "TaxEnergyOnAccountAmount")]
    pub tax_energy_on_account_amount: Option<cct::Amount>,
/// The monetary amount of the balance of taxes owing.
    #[serde(default, rename = "TaxEnergyBalanceAmount")]
    pub tax_energy_balance_amount: Option<cct::Amount>,
/// The relevant taxation scheme.
    #[serde(rename = "TaxScheme")]
    pub tax_scheme: crate::TaxScheme,
}
