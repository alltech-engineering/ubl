use serde::{Deserialize, Serialize};

pub type DespatchParty = crate::Party;
pub type DespatchPeriod = crate::Period;
pub type DespatchTransportationService = crate::TransportationService;

include!("line.rs");

#[derive(Debug, Deserialize, Serialize)]
pub struct Despatch {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
    #[serde(default, rename = "RequestedDespatchDate")]
    pub requested_despatch_date: Option<udt::DateTime>,
    #[serde(default, rename = "RequestedDespatchTime")]
    pub requested_despatch_time: Option<udt::DateTime>,
    #[serde(default, rename = "EstimatedDespatchDate")]
    pub estimated_despatch_date: Option<udt::DateTime>,
    #[serde(default, rename = "EstimatedDespatchTime")]
    pub estimated_despatch_time: Option<udt::DateTime>,
    #[serde(default, rename = "ActualDespatchDate")]
    pub actual_despatch_date: Option<udt::DateTime>,
    #[serde(default, rename = "ActualDespatchTime")]
    pub actual_despatch_time: Option<udt::DateTime>,
    #[serde(default, rename = "GuaranteedDespatchDate")]
    pub guaranteed_despatch_date: Option<udt::DateTime>,
    #[serde(default, rename = "GuaranteedDespatchTime")]
    pub guaranteed_despatch_time: Option<udt::DateTime>,
    #[serde(default, rename = "ReleaseID")]
    pub release_id: Option<cct::Identifier>,
    #[serde(default, rename = "Instructions")]
    pub instructions: Vec<cct::Text>,
    #[serde(default, rename = "DespatchAddress")]
    pub despatch_address: Option<crate::Address>,
    #[serde(default, rename = "DespatchLocation")]
    pub despatch_location: Option<crate::Location>,
    #[serde(default, rename = "DespatchParty")]
    pub despatch_party: Option<crate::Party>,
    #[serde(default, rename = "CarrierParty")]
    pub carrier_party: Option<crate::Party>,
    #[serde(default, rename = "NotifyParty")]
    pub notify_party: Vec<crate::Party>,
    #[serde(default, rename = "ResponsibleParty")]
    pub responsible_party: Option<crate::Party>,
    #[serde(default, rename = "Contact")]
    pub contact: Option<crate::Contact>,
    #[serde(default, rename = "EstimatedDespatchPeriod")]
    pub estimated_despatch_period: Option<crate::Period>,
    #[serde(default, rename = "RequestedDespatchPeriod")]
    pub requested_despatch_period: Option<crate::Period>,
}
