#[derive(Debug, Deserialize, Serialize)]
pub struct CompletedTask {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "AnnualAverageAmount")]
    pub annual_average_amount: Option<super::cct::AmountType>,
    #[serde(default, rename = "TotalTaskAmount")]
    pub total_task_amount: Option<super::cct::AmountType>,
    #[serde(default, rename = "PartyCapacityAmount")]
    pub party_capacity_amount: Option<super::cct::AmountType>,
    #[serde(default, rename = "Description")]
    pub description: Vec<super::cct::TextType>,
    #[serde(default, rename = "EvidenceSupplied")]
    pub evidence_supplied: Vec<EvidenceSupplied>,
    #[serde(default, rename = "SuppliedEvidence")]
    pub supplied_evidence: Vec<Evidence>,
    #[serde(default, rename = "Period")]
    pub period: Option<Period>,
    #[serde(default, rename = "RecipientCustomerParty")]
    pub recipient_customer_party: Option<CustomerParty>,
}
