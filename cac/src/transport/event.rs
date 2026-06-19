#[derive(Debug, Deserialize, Serialize)]
pub struct TransportEvent {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "IdentificationID")]
    pub identification_id: Option<cct::Identifier>,
    #[serde(default, rename = "OccurrenceDate")]
    pub occurrence_date: Option<udt::DateTime>,
    #[serde(default, rename = "OccurrenceTime")]
    pub occurrence_time: Option<udt::DateTime>,
    #[serde(default, rename = "TransportEventTypeCode")]
    pub transport_event_type_code: Option<cct::Code>,
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
    #[serde(default, rename = "CompletionIndicator")]
    pub completion_indicator: Option<udt::Indicator>,
    #[serde(default, rename = "ReportedShipment")]
    pub reported_shipment: Option<Box<crate::Shipment>>,
    #[serde(default, rename = "CurrentStatus")]
    pub current_status: Vec<crate::Status>,
    #[serde(default, rename = "ResponsibleParty")]
    pub responsible_party: Option<crate::Party>,
    #[serde(default, rename = "Contact")]
    pub contact: Vec<crate::Contact>,
    #[serde(default, rename = "Location")]
    pub location: Option<crate::Location>,
    #[serde(default, rename = "Signature")]
    pub signature: Option<crate::Signature>,
    #[serde(default, rename = "Period")]
    pub period: Vec<crate::Period>,
}
