#[derive(Debug, Deserialize, Serialize)]
pub struct HazardousGoodsTransit {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "TransportEmergencyCardCode")]
    pub transport_emergency_card_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "PackingCriteriaCode")]
    pub packing_criteria_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "HazardousRegulationCode")]
    pub hazardous_regulation_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "InhalationToxicityZoneCode")]
    pub inhalation_toxicity_zone_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "TransportAuthorizationCode")]
    pub transport_authorization_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "TransitDescription")]
    pub transit_description: Vec<super::cct::TextType>,
    #[serde(default, rename = "MaximumTemperature")]
    pub maximum_temperature: Option<Temperature>,
    #[serde(default, rename = "MinimumTemperature")]
    pub minimum_temperature: Option<Temperature>,
}
