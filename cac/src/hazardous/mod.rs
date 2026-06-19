use serde::{Deserialize, Serialize};


include!("item.rs");

#[derive(Debug, Deserialize, Serialize)]
pub struct HazardousGoodsTransit {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "TransportEmergencyCardCode")]
    pub transport_emergency_card_code: Option<cct::Code>,
    #[serde(default, rename = "PackingCriteriaCode")]
    pub packing_criteria_code: Option<cct::Code>,
    #[serde(default, rename = "HazardousRegulationCode")]
    pub hazardous_regulation_code: Option<cct::Code>,
    #[serde(default, rename = "InhalationToxicityZoneCode")]
    pub inhalation_toxicity_zone_code: Option<cct::Code>,
    #[serde(default, rename = "TransportAuthorizationCode")]
    pub transport_authorization_code: Option<cct::Code>,
    #[serde(default, rename = "TransitDescription")]
    pub transit_description: Vec<cct::Text>,
    #[serde(default, rename = "MaximumTemperature")]
    pub maximum_temperature: Option<crate::Temperature>,
    #[serde(default, rename = "MinimumTemperature")]
    pub minimum_temperature: Option<crate::Temperature>,
}
