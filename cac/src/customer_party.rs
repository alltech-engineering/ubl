#[derive(Debug, Deserialize, Serialize)]
/// A class to describe a customer party.
///
/// UBL Dictionary Entry Name: `Customer Party. Details`
///
/// Generated from XSD type `CustomerPartyType`.
pub struct CustomerParty {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// An identifier for the customer's account, assigned by the customer itself.
    #[serde(default, rename = "CustomerAssignedAccountID")]
    pub customer_assigned_account_id: Option<cct::Identifier>,
/// An identifier for the customer's account, assigned by the supplier.
    #[serde(default, rename = "SupplierAssignedAccountID")]
    pub supplier_assigned_account_id: Option<cct::Identifier>,
/// An identifier for the customer's account, assigned by a third party.
    #[serde(default, rename = "AdditionalAccountID")]
    pub additional_account_id: Vec<cct::Identifier>,
/// The Customer Party itself.
    #[serde(default, rename = "Party")]
    pub party: Option<Party>,
/// A customer contact for deliveries.
    #[serde(default, rename = "DeliveryContact")]
    pub delivery_contact: Option<Contact>,
/// A customer contact for accounting.
    #[serde(default, rename = "AccountingContact")]
    pub accounting_contact: Option<Contact>,
/// A customer contact for purchasing.
    #[serde(default, rename = "BuyerContact")]
    pub buyer_contact: Option<Contact>,
}
