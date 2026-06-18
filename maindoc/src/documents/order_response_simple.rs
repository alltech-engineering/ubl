#[derive(Debug, Deserialize, Serialize)]
pub struct OrderResponseSimple {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "UBLVersionID")]
    pub ubl_version_id: Option<cct::IdentifierType>,
    #[serde(default, rename = "CustomizationID")]
    pub customization_id: Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileID")]
    pub profile_id: Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileExecutionID")]
    pub profile_execution_id: Option<cct::IdentifierType>,
    #[serde(rename = "ID")]
    pub id: cct::IdentifierType,
    #[serde(default, rename = "CopyIndicator")]
    pub copy_indicator: Option<udt::IndicatorType>,
    #[serde(default, rename = "UUID")]
    pub uuid: Option<cct::IdentifierType>,
    #[serde(rename = "IssueDate")]
    pub issue_date: udt::DateTimeType,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: Option<udt::DateTimeType>,
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::TextType>,
    #[serde(rename = "AcceptedIndicator")]
    pub accepted_indicator: udt::IndicatorType,
    #[serde(default, rename = "RejectionNote")]
    pub rejection_note: Vec<cct::TextType>,
    #[serde(default, rename = "CustomerReference")]
    pub customer_reference: Option<cct::TextType>,
    #[serde(default, rename = "AccountingCostCode")]
    pub accounting_cost_code: Option<cct::CodeType>,
    #[serde(default, rename = "AccountingCost")]
    pub accounting_cost: Option<cct::TextType>,
    #[serde(rename = "OrderReference")]
    pub order_reference: cac::OrderReference,
    #[serde(default, rename = "OrderChangeDocumentReference")]
    pub order_change_document_reference: Vec<cac::DocumentReference>,
    #[serde(default, rename = "AdditionalDocumentReference")]
    pub additional_document_reference: Vec<cac::DocumentReference>,
    #[serde(default, rename = "Signature")]
    pub signature: Vec<cac::Signature>,
    #[serde(rename = "SellerSupplierParty")]
    pub seller_supplier_party: cac::SupplierParty,
    #[serde(rename = "BuyerCustomerParty")]
    pub buyer_customer_party: cac::CustomerParty,
    #[serde(default, rename = "OriginatorCustomerParty")]
    pub originator_customer_party: Option<cac::CustomerParty>,
    #[serde(default, rename = "BeneficiaryParty")]
    pub beneficiary_party: Vec<cac::Party>,
    #[serde(default, rename = "AccountingSupplierParty")]
    pub accounting_supplier_party: Option<cac::SupplierParty>,
    #[serde(default, rename = "AccountingCustomerParty")]
    pub accounting_customer_party: Option<cac::CustomerParty>,
}
