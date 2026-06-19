#[derive(Debug, Deserialize, Serialize)]
/// A class to describe a post award process. These processes following the agreement on a contract for
/// supply of goods or services ( for example, after the awarding of a tender).
///
/// UBL Dictionary Entry Name: `Post Award Process. Details`
///
/// Generated from XSD type `PostAwardProcessType`.
pub struct PostAwardProcess {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
/// An indicator to specify whether an electronic catalogue will be used during the post award phase.
    #[serde(default, rename = "ElectronicCatalogueUsageIndicator")]
    pub electronic_catalogue_usage_indicator: Option<udt::Indicator>,
/// An indicator on whether the electronic invoice is allowed for this process.
    #[serde(default, rename = "ElectronicInvoiceAcceptedIndicator")]
    pub electronic_invoice_accepted_indicator: Option<udt::Indicator>,
/// An indicator on whether electronic ordering will be used in the post award process.
    #[serde(default, rename = "ElectronicOrderUsageIndicator")]
    pub electronic_order_usage_indicator: Option<udt::Indicator>,
/// (Endorsed cardinality: 0..1) An indicator on whether electronic payment will be used in the post
/// award process.
    #[serde(default, rename = "ElectronicPaymentUsageIndicator")]
    pub electronic_payment_usage_indicator: Vec<udt::Indicator>,
}
