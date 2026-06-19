use serde::{Deserialize, Serialize};

include!("operator_short_list.rs");
include!("operator_role.rs");

#[derive(Debug, Deserialize, Serialize)]
pub struct EconomicOperatorParty {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "QualifyingParty")]
    pub qualifying_party: Vec<crate::QualifyingParty>,
    #[serde(default, rename = "EconomicOperatorRole")]
    pub economic_operator_role: Vec<EconomicOperatorRole>,
    #[serde(rename = "Party")]
    pub party: crate::Party,
}
