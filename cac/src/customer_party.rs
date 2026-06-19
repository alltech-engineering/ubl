#[derive(Debug, Deserialize, Serialize)]
pub struct CustomerParty {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
    #[serde(default, rename = "CustomerAssignedAccountID")]
    pub customer_assigned_account_id: Option<cct::Identifier>,
    #[serde(default, rename = "SupplierAssignedAccountID")]
    pub supplier_assigned_account_id: Option<cct::Identifier>,
    #[serde(default, rename = "AdditionalAccountID")]
    pub additional_account_id: Vec<cct::Identifier>,
    #[serde(default, rename = "Party")]
    pub party: Option<Party>,
    #[serde(default, rename = "DeliveryContact")]
    pub delivery_contact: Option<Contact>,
    #[serde(default, rename = "AccountingContact")]
    pub accounting_contact: Option<Contact>,
    #[serde(default, rename = "BuyerContact")]
    pub buyer_contact: Option<Contact>,
}
