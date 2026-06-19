use serde::{Deserialize, Serialize};

include!("waste.rs");
include!("health_declaration.rs");

#[derive(Debug, Deserialize, Serialize)]
/// A class to describe a vessel used for transport by water (including sea, river, and canal).
///
/// UBL Dictionary Entry Name: `Maritime Transport. Details`
///
/// Generated from XSD type `MaritimeTransportType`.
pub struct MaritimeTransport {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// An identifier for a specific vessel.
    #[serde(default, rename = "VesselID")]
    pub vessel_id: Option<cct::Identifier>,
/// The name of the vessel.
    #[serde(default, rename = "VesselName")]
    pub vessel_name: Option<cct::Text>,
/// The radio call sign of the vessel.
    #[serde(default, rename = "RadioCallSignID")]
    pub radio_call_sign_id: Option<cct::Identifier>,
/// A Maritime Mobile Service Identity (MMSI) required for this vessel.
    #[serde(default, rename = "MMSIRegistrationID")]
    pub mmsi_registration_id: Option<cct::Identifier>,
/// Information about what services a vessel will require when it arrives at a port, such as refueling,
/// maintenance, waste disposal etc.
    #[serde(default, rename = "ShipsRequirements")]
    pub ships_requirements: Vec<cct::Text>,
/// Gross tonnage is calculated by measuring a ship's volume (from keel to funnel, to the outside of the
/// hull framing) and applying a mathematical formula and is used to determine things such as a ship's
/// manning regulations, safety rules, registration fees and port dues.
    #[serde(default, rename = "GrossTonnageMeasure")]
    pub gross_tonnage_measure: Option<cct::Measure>,
/// Net tonnage is calculated by measuring a ship's internal volume and applying a mathematical formula
/// and is used to calculate the port duties.
    #[serde(default, rename = "NetTonnageMeasure")]
    pub net_tonnage_measure: Option<cct::Measure>,
/// The measure of the segregated ballast of the vessel.
    #[serde(default, rename = "SegregatedBallastMeasure")]
    pub segregated_ballast_measure: Option<cct::Measure>,
/// A code specifying the ship configuration.
    #[serde(default, rename = "ShipConfigurationCode")]
    pub ship_configuration_code: Option<cct::Code>,
/// A code specifying the irradiated nuclear fuel (INF) ship class.
    #[serde(default, rename = "INFShipClassCode")]
    pub inf_ship_class_code: Option<cct::Code>,
/// The locus or exact location of the antenna on the vessel
    #[serde(default, rename = "AntennaLocus")]
    pub antenna_locus: Option<cct::Text>,
/// The certificate issued to the ship by the ships registry in a given flag state.
    #[serde(default, rename = "RegistryCertificateDocumentReference")]
    pub registry_certificate_document_reference: Option<crate::DocumentReference>,
/// The port in which a vessel is registered or permanently based.
    #[serde(default, rename = "RegistryPortLocation")]
    pub registry_port_location: Option<crate::Location>,
/// The vessel dynamics for this maritime transport.
    #[serde(default, rename = "VesselDynamics")]
    pub vessel_dynamics: Option<crate::VesselDynamics>,
}
