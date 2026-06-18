#[derive(Debug, Deserialize, Serialize)]
pub struct SupplierParty {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "CustomerAssignedAccountID")]
    pub customer_assigned_account_id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "AdditionalAccountID")]
    pub additional_account_id: Vec<super::cct::IdentifierType>,
    #[serde(default, rename = "DataSendingCapability")]
    pub data_sending_capability: Option<super::cct::TextType>,
    #[serde(default, rename = "Party")]
    pub party: Option<Party>,
    #[serde(default, rename = "DespatchContact")]
    pub despatch_contact: Option<Contact>,
    #[serde(default, rename = "AccountingContact")]
    pub accounting_contact: Option<Contact>,
    #[serde(default, rename = "SellerContact")]
    pub seller_contact: Option<Contact>,
}
