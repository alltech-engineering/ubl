#[derive(Debug, Deserialize, Serialize)]
pub struct DeliveryNote {
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
    #[serde(default, rename = "UUID")]
    pub uuid: Option<cct::Identifier>,
    #[serde(rename = "IssueDate")]
    pub issue_date: udt::DateTime,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: Option<udt::DateTime>,
    #[serde(default, rename = "DocumentStatusCode")]
    pub document_status_code: Option<cct::Code>,
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
    #[serde(default, rename = "AccountingCostCode")]
    pub accounting_cost_code: Option<cct::Code>,
    #[serde(default, rename = "AccountingCost")]
    pub accounting_cost: Option<cct::Text>,
    #[serde(default, rename = "LineCountNumeric")]
    pub line_count_numeric: Option<cct::Numeric>,
    #[serde(default, rename = "OrderReference")]
    pub order_reference: Vec<cac::OrderReference>,
    #[serde(default, rename = "ProjectReference")]
    pub project_reference: Vec<cac::ProjectReference>,
    #[serde(default, rename = "AdditionalDocumentReference")]
    pub additional_document_reference: Vec<cac::DocumentReference>,
    #[serde(default, rename = "Signature")]
    pub signature: Vec<cac::Signature>,
    #[serde(rename = "DespatchSupplierParty")]
    pub despatch_supplier_party: cac::SupplierParty,
    #[serde(rename = "DeliveryCustomerParty")]
    pub delivery_customer_party: cac::CustomerParty,
    #[serde(default, rename = "BuyerCustomerParty")]
    pub buyer_customer_party: Option<cac::CustomerParty>,
    #[serde(default, rename = "SellerSupplierParty")]
    pub seller_supplier_party: Option<cac::SupplierParty>,
    #[serde(default, rename = "OriginatorCustomerParty")]
    pub originator_customer_party: Option<cac::CustomerParty>,
    #[serde(default, rename = "BeneficiaryParty")]
    pub beneficiary_party: Vec<cac::Party>,
    #[serde(default, rename = "Shipment")]
    pub shipment: Option<cac::Shipment>,
    #[serde(default, rename = "DespatchLine")]
    pub despatch_line: Vec<cac::DespatchLine>,
}
