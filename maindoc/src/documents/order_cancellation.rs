#[derive(Debug, Deserialize, Serialize)]
pub struct OrderCancellation {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "UBLVersionID")]
    pub ubl_version_id: Option<cct::Identifier>,
    #[serde(default, rename = "CustomizationID")]
    pub customization_id: Option<cct::Identifier>,
    #[serde(default, rename = "ProfileID")]
    pub profile_id: Option<cct::Identifier>,
    #[serde(default, rename = "ProfileExecutionID")]
    pub profile_execution_id: Option<cct::Identifier>,
    #[serde(rename = "ID")]
    pub id: cct::Identifier,
    #[serde(default, rename = "CopyIndicator")]
    pub copy_indicator: Option<udt::Indicator>,
    #[serde(default, rename = "UUID")]
    pub uuid: Option<cct::Identifier>,
    #[serde(rename = "IssueDate")]
    pub issue_date: udt::DateTime,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: Option<udt::DateTime>,
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
    #[serde(default, rename = "CancellationNote")]
    pub cancellation_note: Vec<cct::Text>,
    #[serde(default, rename = "OrderReference")]
    pub order_reference: Vec<cac::OrderReference>,
    #[serde(default, rename = "OriginatorDocumentReference")]
    pub originator_document_reference: Option<cac::DocumentReference>,
    #[serde(default, rename = "AdditionalDocumentReference")]
    pub additional_document_reference: Vec<cac::DocumentReference>,
    #[serde(default, rename = "Contract")]
    pub contract: Vec<cac::Contract>,
    #[serde(default, rename = "Signature")]
    pub signature: Vec<cac::Signature>,
    #[serde(rename = "BuyerCustomerParty")]
    pub buyer_customer_party: cac::CustomerParty,
    #[serde(rename = "SellerSupplierParty")]
    pub seller_supplier_party: cac::SupplierParty,
    #[serde(default, rename = "OriginatorCustomerParty")]
    pub originator_customer_party: Option<cac::CustomerParty>,
    #[serde(default, rename = "BeneficiaryParty")]
    pub beneficiary_party: Vec<cac::Party>,
}
