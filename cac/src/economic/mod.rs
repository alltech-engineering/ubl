use serde::{Deserialize, Serialize};

include!("operator_short_list.rs");
include!("operator_role.rs");

#[derive(Debug, Deserialize, Serialize)]
/// A class to describe a potential contractor, supplier and service provider responding to a tender.
///
/// UBL Dictionary Entry Name: `Economic Operator Party. Details`
///
/// Generated from XSD type `EconomicOperatorPartyType`.
pub struct EconomicOperatorParty {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// The party qualifying this economic operator.
    #[serde(default, rename = "QualifyingParty")]
    pub qualifying_party: Vec<crate::QualifyingParty>,
/// The role of the party in a tender consortium.
    #[serde(default, rename = "EconomicOperatorRole")]
    pub economic_operator_role: Vec<EconomicOperatorRole>,
/// The party information about the economic operator in a tender.
    #[serde(rename = "Party")]
    pub party: crate::Party,
}
