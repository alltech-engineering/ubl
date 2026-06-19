#[derive(Debug, Deserialize, Serialize)]
pub struct InsurancePolicy {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
    #[serde(default, rename = "InsuranceTypeCode")]
    pub insurance_type_code: Option<cct::Code>,
    #[serde(default, rename = "InsuranceTypeDescription")]
    pub insurance_type_description: Vec<cct::Text>,
    #[serde(default, rename = "InsuredValueAmount")]
    pub insured_value_amount: Option<cct::Amount>,
    #[serde(default, rename = "DeductibleAmount")]
    pub deductible_amount: Option<cct::Amount>,
    #[serde(default, rename = "ExcessAmount")]
    pub excess_amount: Option<cct::Amount>,
    #[serde(default, rename = "InsurancePremiumAmount")]
    pub insurance_premium_amount: Option<cct::Amount>,
    #[serde(default, rename = "InsurerParty")]
    pub insurer_party: Option<Party>,
    #[serde(default, rename = "BrokerParty")]
    pub broker_party: Option<Party>,
    #[serde(default, rename = "PolicyHolderParty")]
    pub policy_holder_party: Option<Party>,
    #[serde(default, rename = "BeneficiaryParty")]
    pub beneficiary_party: Vec<Party>,
    #[serde(default, rename = "PolicyDocumentReference")]
    pub policy_document_reference: Option<DocumentReference>,
}
