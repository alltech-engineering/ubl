#[derive(Debug, Deserialize, Serialize)]
pub struct TransportEvent {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "IdentificationID")]
    pub identification_id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "OccurrenceDate")]
    pub occurrence_date: Option<super::udt::DateTimeType>,
    #[serde(default, rename = "OccurrenceTime")]
    pub occurrence_time: Option<super::udt::DateTimeType>,
    #[serde(default, rename = "TransportEventTypeCode")]
    pub transport_event_type_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "Description")]
    pub description: Vec<super::cct::TextType>,
    #[serde(default, rename = "CompletionIndicator")]
    pub completion_indicator: Option<super::udt::IndicatorType>,
    #[serde(default, rename = "ReportedShipment")]
    pub reported_shipment: Option<Box<Shipment>>,
    #[serde(default, rename = "CurrentStatus")]
    pub current_status: Vec<Status>,
    #[serde(default, rename = "ResponsibleParty")]
    pub responsible_party: Option<Party>,
    #[serde(default, rename = "Contact")]
    pub contact: Vec<Contact>,
    #[serde(default, rename = "Location")]
    pub location: Option<Location>,
    #[serde(default, rename = "Signature")]
    pub signature: Option<Signature>,
    #[serde(default, rename = "Period")]
    pub period: Vec<Period>,
}
