use serde::{Deserialize, Serialize};


include!("extension.rs");
include!("execution_requirement.rs");

#[derive(Debug, Deserialize, Serialize)]
/// A class to describe a contract.
///
/// UBL Dictionary Entry Name: `Contract. Details`
///
/// Generated from XSD type `ContractType`.
pub struct Contract {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
/// An identifier for this contract.
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
/// The date on which this contract was issued.
    #[serde(default, rename = "IssueDate")]
    pub issue_date: Option<udt::DateTime>,
/// The time at which this contract was issued.
    #[serde(default, rename = "IssueTime")]
    pub issue_time: Option<udt::DateTime>,
/// In a transportation contract, the deadline date by which the services referred to in the transport
/// execution plan have to be booked. For example, if this service is a carrier service scheduled for
/// Wednesday 16 February 2011 at 10 a.m. CET, the nomination date might be Tuesday15 February 2011.
    #[serde(default, rename = "NominationDate")]
    pub nomination_date: Option<udt::DateTime>,
/// In a transportation contract, the deadline time by which the services referred to in the transport
/// execution plan have to be booked. For example, if this service is a carrier service scheduled for
/// Wednesday 16 February 2011 at 10 a.m. CET, the nomination date might be Tuesday15 February 2011 and
/// the nomination time 4 p.m. at the latest.
    #[serde(default, rename = "NominationTime")]
    pub nomination_time: Option<udt::DateTime>,
/// The type of this contract, expressed as a code, such as "Cost plus award fee" and "Cost plus fixed
/// fee" from UNCEFACT Contract Type code list.
    #[serde(default, rename = "ContractTypeCode")]
    pub contract_type_code: Option<cct::Code>,
/// The type of this contract, expressed as text, such as "Cost plus award fee" and "Cost plus fixed
/// fee" from UNCEFACT Contract Type code list.
    #[serde(default, rename = "ContractType")]
    pub contract_type: Option<cct::Text>,
/// Free-form text conveying information that is not contained explicitly in other structures.
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
/// An identifier for the current version of this contract.
    #[serde(default, rename = "VersionID")]
    pub version_id: Option<cct::Identifier>,
/// The main reason for modifying the contract expressed as a code.
    #[serde(default, rename = "ModificationReasonCode")]
    pub modification_reason_code: Option<cct::Code>,
/// Text describing the main reason for modifying the contract
    #[serde(default, rename = "ModificationReasonDescription")]
    pub modification_reason_description: Vec<cct::Text>,
/// Text describing this contract.
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
/// The period during which this contract is valid.
    #[serde(default, rename = "ValidityPeriod")]
    pub validity_period: Option<crate::Period>,
/// A reference to a contract document.
    #[serde(default, rename = "ContractDocumentReference")]
    pub contract_document_reference: Vec<crate::DocumentReference>,
/// In a transportation contract, the period required to book the services specified in the contract
/// before the services can begin.
    #[serde(default, rename = "NominationPeriod")]
    pub nomination_period: Option<crate::Period>,
/// In a transportation contract, the delivery of the services required to book the services specified
/// in the contract.
    #[serde(default, rename = "ContractualDelivery")]
    pub contractual_delivery: Option<crate::Delivery>,
}
