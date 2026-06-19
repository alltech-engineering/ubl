#[derive(Debug, Deserialize, Serialize)]
/// A record for a ship call at a port facility.
///
/// UBL Dictionary Entry Name: `Port Call Record. Details`
///
/// Generated from XSD type `PortCallRecordType`.
pub struct PortCallRecord {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
/// An identifier for this port call record.
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
/// A code describing the security level of the port facility call record.
    #[serde(default, rename = "SecurityLevelCode")]
    pub security_level_code: Option<cct::Code>,
/// One or more security measures applied to this port call record.
    #[serde(default, rename = "SecurityMeasure")]
    pub security_measure: Vec<crate::SecurityMeasure>,
/// The location of the port facility.
    #[serde(default, rename = "PortFacilityLocation")]
    pub port_facility_location: Option<crate::Location>,
/// The period when this port call took place.
    #[serde(default, rename = "Period")]
    pub period: Option<crate::Period>,
}
