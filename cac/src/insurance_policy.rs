#[derive(Debug, Deserialize, Serialize)]
/// A class to define an insurance policy.
///
/// UBL Dictionary Entry Name: `Insurance Policy. Details`
///
/// Generated from XSD type `InsurancePolicyType`.
pub struct InsurancePolicy {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
/// An identifier for this insurance policy, such as the policy number.
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
/// A code describing the type of insurance under this policy.
    #[serde(default, rename = "InsuranceTypeCode")]
    pub insurance_type_code: Option<cct::Code>,
/// A textual description of the type of insurance under this policy.
    #[serde(default, rename = "InsuranceTypeDescription")]
    pub insurance_type_description: Vec<cct::Text>,
/// The amount covered by this Insurance Policy.
    #[serde(default, rename = "InsuredValueAmount")]
    pub insured_value_amount: Option<cct::Amount>,
/// The deductible amount specified in the policy of this Insurance.
    #[serde(default, rename = "DeductibleAmount")]
    pub deductible_amount: Option<cct::Amount>,
/// The excess amount specified in the policy of this Insurance.
    #[serde(default, rename = "ExcessAmount")]
    pub excess_amount: Option<cct::Amount>,
/// The amount of the premium payable to an insurance company under this Insurance Policy.
    #[serde(default, rename = "InsurancePremiumAmount")]
    pub insurance_premium_amount: Option<cct::Amount>,
/// The Party providing the insurance under this Insurance Policy.
    #[serde(default, rename = "InsurerParty")]
    pub insurer_party: Option<Party>,
/// Intermediary agent or broker of the insurance under this Insurance Policy.
    #[serde(default, rename = "BrokerParty")]
    pub broker_party: Option<Party>,
/// The Party holding this Insurance Policy.
    #[serde(default, rename = "PolicyHolderParty")]
    pub policy_holder_party: Option<Party>,
/// A Party entitled to benefit from this Insurance Policy.
    #[serde(default, rename = "BeneficiaryParty")]
    pub beneficiary_party: Vec<Party>,
/// A reference to the policy document.
    #[serde(default, rename = "PolicyDocumentReference")]
    pub policy_document_reference: Option<DocumentReference>,
}
