#[derive(Debug, Deserialize, Serialize)]
pub struct UtilityStatement {
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
    #[serde(rename = "UtilityStatementTypeCode")]
    pub utility_statement_type_code: cct::Code,
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
    #[serde(rename = "DocumentCurrencyCode")]
    pub document_currency_code: cct::Code,
    #[serde(default, rename = "AccountingCostCode")]
    pub accounting_cost_code: Option<cct::Code>,
    #[serde(default, rename = "AccountingCost")]
    pub accounting_cost: Option<cct::Text>,
    #[serde(rename = "ParentDocumentReference")]
    pub parent_document_reference: cac::DocumentReference,
    #[serde(default, rename = "AdditionalDocumentReference")]
    pub additional_document_reference: Vec<cac::DocumentReference>,
    #[serde(default, rename = "Signature")]
    pub signature: Vec<cac::Signature>,
    #[serde(rename = "SenderParty")]
    pub sender_party: cac::Party,
    #[serde(rename = "ReceiverParty")]
    pub receiver_party: cac::Party,
    #[serde(default, rename = "CustomerParty")]
    pub customer_party: Option<cac::CustomerParty>,
    #[serde(default, rename = "SubscriberParty")]
    pub subscriber_party: Option<cac::Party>,
    #[serde(default, rename = "MainOnAccountPayment")]
    pub main_on_account_payment: Vec<cac::OnAccountPayment>,
    #[serde(default, rename = "SubscriberConsumption")]
    pub subscriber_consumption: Vec<cac::SubscriberConsumption>,
}
