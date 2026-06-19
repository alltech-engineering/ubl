use serde::{Deserialize, Serialize};

pub type ServiceDescriptionDocumentReference = crate::DocumentReference;
pub type ServiceDescriptionRequestDocumentReference = crate::DocumentReference;
pub type ServiceProviderResponseDeadlinePeriod = crate::Period;
pub type ServiceProviderResponseRequiredPeriod = crate::Period;

include!("frequency.rs");
include!("level_agreement.rs");

#[derive(Debug, Deserialize, Serialize)]
pub struct ServiceProviderParty {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
    #[serde(default, rename = "ServiceTypeCode")]
    pub service_type_code: Option<cct::Code>,
    #[serde(default, rename = "ServiceType")]
    pub service_type: Vec<cct::Text>,
    #[serde(rename = "Party")]
    pub party: Box<crate::Party>,
    #[serde(default, rename = "SellerContact")]
    pub seller_contact: Option<crate::Contact>,
}
