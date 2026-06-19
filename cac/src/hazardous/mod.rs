use serde::{Deserialize, Serialize};


include!("item.rs");

#[derive(Debug, Deserialize, Serialize)]
/// A class to describe hazardous goods in transit.
///
/// UBL Dictionary Entry Name: `Hazardous Goods Transit. Details`
///
/// Generated from XSD type `HazardousGoodsTransitType`.
pub struct HazardousGoodsTransit {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
/// An identifier for a transport emergency card describing the actions to be taken in an emergency in
/// transporting the hazardous goods. It may be the identity number of a hazardous emergency response
/// plan assigned by the appropriate authority.
    #[serde(default, rename = "TransportEmergencyCardCode")]
    pub transport_emergency_card_code: Option<cct::Code>,
/// A code signifying the packaging requirement for transportation of the hazardous goods as assigned by
/// IATA, IMDB, ADR, RID etc.
    #[serde(default, rename = "PackingCriteriaCode")]
    pub packing_criteria_code: Option<cct::Code>,
/// A code signifying the set of legal regulations governing the transportation of the hazardous goods.
    #[serde(default, rename = "HazardousRegulationCode")]
    pub hazardous_regulation_code: Option<cct::Code>,
/// A code signifying the Inhalation Toxicity Hazard Zone for the hazardous goods, as defined by the US
/// Department of Transportation.
    #[serde(default, rename = "InhalationToxicityZoneCode")]
    pub inhalation_toxicity_zone_code: Option<cct::Code>,
/// A code signifying authorization for the transportation of hazardous cargo.
    #[serde(default, rename = "TransportAuthorizationCode")]
    pub transport_authorization_code: Option<cct::Code>,
/// A textual description of this hazardous goods transit.
    #[serde(default, rename = "TransitDescription")]
    pub transit_description: Vec<cct::Text>,
/// The maximum temperature at which the hazardous goods can safely be transported.
    #[serde(default, rename = "MaximumTemperature")]
    pub maximum_temperature: Option<crate::Temperature>,
/// The minimum temperature at which the hazardous goods can safely be transported.
    #[serde(default, rename = "MinimumTemperature")]
    pub minimum_temperature: Option<crate::Temperature>,
}
