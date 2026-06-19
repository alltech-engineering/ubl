#[derive(Debug, Deserialize, Serialize)]
/// A class to describe a ship to ship activity record.
///
/// UBL Dictionary Entry Name: `Ship To Ship Activity Record. Details`
///
/// Generated from XSD type `ShipToShipActivityRecordType`.
pub struct ShipToShipActivityRecord {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
/// An indentifier for this ship to ship activity.
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
/// A text describing the ship to ship activity.
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
/// Any security measures applied to this ship to ship activity in lieu of those specified in the
/// approved Ship Security Plan (SSP).
    #[serde(default, rename = "AppliedSecurityMeasure")]
    pub applied_security_measure: Vec<crate::SecurityMeasure>,
/// The duration of this ship to ship activity.
    #[serde(default, rename = "Period")]
    pub period: Option<crate::Period>,
/// The location where this ship to ship activity took place.
    #[serde(default, rename = "Location")]
    pub location: Option<crate::Location>,
}
