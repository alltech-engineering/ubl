#[derive(Debug, Deserialize, Serialize)]
pub struct PostAwardProcess {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "ElectronicCatalogueUsageIndicator")]
    pub electronic_catalogue_usage_indicator: Option<udt::Indicator>,
    #[serde(default, rename = "ElectronicInvoiceAcceptedIndicator")]
    pub electronic_invoice_accepted_indicator: Option<udt::Indicator>,
    #[serde(default, rename = "ElectronicOrderUsageIndicator")]
    pub electronic_order_usage_indicator: Option<udt::Indicator>,
    #[serde(default, rename = "ElectronicPaymentUsageIndicator")]
    pub electronic_payment_usage_indicator: Vec<udt::Indicator>,
}
