#[derive(Debug, Deserialize, Serialize)]
/// A class to describe the completion of a specific task in the tendering process.
///
/// UBL Dictionary Entry Name: `Completed Task. Details`
///
/// Generated from XSD type `CompletedTaskType`.
pub struct CompletedTask {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// The average monetary amount of a task such as this completed task.
    #[serde(default, rename = "AnnualAverageAmount")]
    pub annual_average_amount: Option<cct::Amount>,
/// The actual total monetary amount of this completed task.
    #[serde(default, rename = "TotalTaskAmount")]
    pub total_task_amount: Option<cct::Amount>,
/// A monetary amount corresponding to the financial capacity of the party that carried out this
/// completed task.
    #[serde(default, rename = "PartyCapacityAmount")]
    pub party_capacity_amount: Option<cct::Amount>,
/// Text describing this completed task.
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
/// (Deprecated) The evidence justifying a designation of "complete" for this task.
    #[serde(default, rename = "EvidenceSupplied")]
    pub evidence_supplied: Vec<EvidenceSupplied>,
/// The Evidence justifying a designation of "complete" for this task.
    #[serde(default, rename = "SuppliedEvidence")]
    pub supplied_evidence: Vec<Evidence>,
/// The period in which this completed task was performed.
    #[serde(default, rename = "Period")]
    pub period: Option<Period>,
/// The original customer for this completed task.
    #[serde(default, rename = "RecipientCustomerParty")]
    pub recipient_customer_party: Option<CustomerParty>,
}
