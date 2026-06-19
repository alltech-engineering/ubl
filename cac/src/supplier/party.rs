#[derive(Debug, Deserialize, Serialize)]
/// A class to describe a supplier party.
///
/// UBL Dictionary Entry Name: `Supplier Party. Details`
///
/// Generated from XSD type `SupplierPartyType`.
pub struct SupplierParty {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// An identifier for this supplier party, assigned by the customer.
    #[serde(default, rename = "CustomerAssignedAccountID")]
    pub customer_assigned_account_id: Option<cct::Identifier>,
/// An additional identifier for this supplier party.
    #[serde(default, rename = "AdditionalAccountID")]
    pub additional_account_id: Vec<cct::Identifier>,
/// Text describing the supplier's ability to send invoice data via a purchase card provider (e.g.,
/// VISA, MasterCard, American Express).
    #[serde(default, rename = "DataSendingCapability")]
    pub data_sending_capability: Option<cct::Text>,
/// The Supplier Party itself.
    #[serde(default, rename = "Party")]
    pub party: Option<crate::Party>,
/// A contact at this supplier party for despatches (pickups).
    #[serde(default, rename = "DespatchContact")]
    pub despatch_contact: Option<crate::Contact>,
/// A contact at this supplier party for accounting.
    #[serde(default, rename = "AccountingContact")]
    pub accounting_contact: Option<crate::Contact>,
/// The primary contact for this supplier party.
    #[serde(default, rename = "SellerContact")]
    pub seller_contact: Option<crate::Contact>,
}
