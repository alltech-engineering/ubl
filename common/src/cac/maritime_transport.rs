#[derive(Debug, Deserialize, Serialize)]
pub struct MaritimeTransport {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "VesselID")]
    pub vessel_id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "VesselName")]
    pub vessel_name: Option<super::cct::TextType>,
    #[serde(default, rename = "RadioCallSignID")]
    pub radio_call_sign_id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "MMSIRegistrationID")]
    pub mmsi_registration_id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "ShipsRequirements")]
    pub ships_requirements: Vec<super::cct::TextType>,
    #[serde(default, rename = "GrossTonnageMeasure")]
    pub gross_tonnage_measure: Option<super::cct::MeasureType>,
    #[serde(default, rename = "NetTonnageMeasure")]
    pub net_tonnage_measure: Option<super::cct::MeasureType>,
    #[serde(default, rename = "SegregatedBallastMeasure")]
    pub segregated_ballast_measure: Option<super::cct::MeasureType>,
    #[serde(default, rename = "ShipConfigurationCode")]
    pub ship_configuration_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "INFShipClassCode")]
    pub inf_ship_class_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "AntennaLocus")]
    pub antenna_locus: Option<super::cct::TextType>,
    #[serde(default, rename = "RegistryCertificateDocumentReference")]
    pub registry_certificate_document_reference: Option<DocumentReference>,
    #[serde(default, rename = "RegistryPortLocation")]
    pub registry_port_location: Option<Location>,
    #[serde(default, rename = "VesselDynamics")]
    pub vessel_dynamics: Option<VesselDynamics>,
}
