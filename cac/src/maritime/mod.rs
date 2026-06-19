use serde::{Deserialize, Serialize};

include!("waste.rs");
include!("health_declaration.rs");

#[derive(Debug, Deserialize, Serialize)]
pub struct MaritimeTransport {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "VesselID")]
    pub vessel_id: Option<cct::Identifier>,
    #[serde(default, rename = "VesselName")]
    pub vessel_name: Option<cct::Text>,
    #[serde(default, rename = "RadioCallSignID")]
    pub radio_call_sign_id: Option<cct::Identifier>,
    #[serde(default, rename = "MMSIRegistrationID")]
    pub mmsi_registration_id: Option<cct::Identifier>,
    #[serde(default, rename = "ShipsRequirements")]
    pub ships_requirements: Vec<cct::Text>,
    #[serde(default, rename = "GrossTonnageMeasure")]
    pub gross_tonnage_measure: Option<cct::Measure>,
    #[serde(default, rename = "NetTonnageMeasure")]
    pub net_tonnage_measure: Option<cct::Measure>,
    #[serde(default, rename = "SegregatedBallastMeasure")]
    pub segregated_ballast_measure: Option<cct::Measure>,
    #[serde(default, rename = "ShipConfigurationCode")]
    pub ship_configuration_code: Option<cct::Code>,
    #[serde(default, rename = "INFShipClassCode")]
    pub inf_ship_class_code: Option<cct::Code>,
    #[serde(default, rename = "AntennaLocus")]
    pub antenna_locus: Option<cct::Text>,
    #[serde(default, rename = "RegistryCertificateDocumentReference")]
    pub registry_certificate_document_reference: Option<crate::DocumentReference>,
    #[serde(default, rename = "RegistryPortLocation")]
    pub registry_port_location: Option<crate::Location>,
    #[serde(default, rename = "VesselDynamics")]
    pub vessel_dynamics: Option<crate::VesselDynamics>,
}
