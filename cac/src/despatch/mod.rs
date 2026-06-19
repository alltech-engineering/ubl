use serde::{Deserialize, Serialize};

pub type DespatchParty = crate::Party;
pub type DespatchPeriod = crate::Period;
pub type DespatchTransportationService = crate::TransportationService;

include!("line.rs");

#[derive(Debug, Deserialize, Serialize)]
/// A class to describe the despatching of goods (their pickup for delivery).
///
/// UBL Dictionary Entry Name: `Despatch. Details`
///
/// Generated from XSD type `DespatchType`.
pub struct Despatch {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
/// An identifier for this despatch event.
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
/// The despatch (pickup) date requested, normally by the buyer.
    #[serde(default, rename = "RequestedDespatchDate")]
    pub requested_despatch_date: Option<udt::DateTime>,
/// The despatch (pickup) time requested, normally by the buyer.
    #[serde(default, rename = "RequestedDespatchTime")]
    pub requested_despatch_time: Option<udt::DateTime>,
/// The estimated despatch (pickup) date.
    #[serde(default, rename = "EstimatedDespatchDate")]
    pub estimated_despatch_date: Option<udt::DateTime>,
/// The estimated despatch (pickup) time.
    #[serde(default, rename = "EstimatedDespatchTime")]
    pub estimated_despatch_time: Option<udt::DateTime>,
/// The actual despatch (pickup) date.
    #[serde(default, rename = "ActualDespatchDate")]
    pub actual_despatch_date: Option<udt::DateTime>,
/// The actual despatch (pickup) time.
    #[serde(default, rename = "ActualDespatchTime")]
    pub actual_despatch_time: Option<udt::DateTime>,
/// The date guaranteed for the despatch (pickup).
    #[serde(default, rename = "GuaranteedDespatchDate")]
    pub guaranteed_despatch_date: Option<udt::DateTime>,
/// The time guaranteed for the despatch (pickup).
    #[serde(default, rename = "GuaranteedDespatchTime")]
    pub guaranteed_despatch_time: Option<udt::DateTime>,
/// An identifier for the release of the despatch used as security control or cargo control (pick-up).
    #[serde(default, rename = "ReleaseID")]
    pub release_id: Option<cct::Identifier>,
/// Text describing any special instructions applying to the despatch (pickup).
    #[serde(default, rename = "Instructions")]
    pub instructions: Vec<cct::Text>,
/// The address of the despatch (pickup).
    #[serde(default, rename = "DespatchAddress")]
    pub despatch_address: Option<crate::Address>,
/// The location of the despatch (pickup).
    #[serde(default, rename = "DespatchLocation")]
    pub despatch_location: Option<crate::Location>,
/// The Party who despatches the goods.
    #[serde(default, rename = "DespatchParty")]
    pub despatch_party: Option<crate::Party>,
/// The Party who provides the transport of goods between named points.
    #[serde(default, rename = "CarrierParty")]
    pub carrier_party: Option<crate::Party>,
/// The Party who is notified of this Despatch.
    #[serde(default, rename = "NotifyParty")]
    pub notify_party: Vec<crate::Party>,
/// The party who picks up the goods.
    #[serde(default, rename = "ResponsibleParty")]
    pub responsible_party: Option<crate::Party>,
/// The primary contact for this despatch (pickup).
    #[serde(default, rename = "Contact")]
    pub contact: Option<crate::Contact>,
/// The period estimated for the despatch (pickup) of goods.
    #[serde(default, rename = "EstimatedDespatchPeriod")]
    pub estimated_despatch_period: Option<crate::Period>,
/// The period requested for the despatch (pickup) of goods.
    #[serde(default, rename = "RequestedDespatchPeriod")]
    pub requested_despatch_period: Option<crate::Period>,
}
