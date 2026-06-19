use serde::{Deserialize, Serialize};

pub type ServiceDescriptionDocumentReference = crate::DocumentReference;
pub type ServiceDescriptionRequestDocumentReference = crate::DocumentReference;
pub type ServiceProviderResponseDeadlinePeriod = crate::Period;
pub type ServiceProviderResponseRequiredPeriod = crate::Period;

include!("frequency.rs");
include!("level_agreement.rs");

#[derive(Debug, Deserialize, Serialize)]
/// A class to describe a party contracting to provide services, such as transportation, finance, etc.
///
/// UBL Dictionary Entry Name: `Service Provider Party. Details`
///
/// Generated from XSD type `ServiceProviderPartyType`.
pub struct ServiceProviderParty {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
/// An identifier for this Service Provider.
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
/// The type of service provided, expressed as a code.
    #[serde(default, rename = "ServiceTypeCode")]
    pub service_type_code: Option<cct::Code>,
/// The type of service provided, expressed as text.
    #[serde(default, rename = "ServiceType")]
    pub service_type: Vec<cct::Text>,
/// The Party who provides this service.
    #[serde(rename = "Party")]
    pub party: Box<crate::Party>,
/// The contact for the Service Provider.
    #[serde(default, rename = "SellerContact")]
    pub seller_contact: Option<crate::Contact>,
}
