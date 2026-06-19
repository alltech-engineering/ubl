#[derive(Debug, Deserialize, Serialize)]
/// A class to describe a significant occurrence or happening related to the transportation of goods.
///
/// UBL Dictionary Entry Name: `Transport Event. Details`
///
/// Generated from XSD type `TransportEventType`.
pub struct TransportEvent {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
/// An identifier for this transport event within an agreed event identification scheme.
    #[serde(default, rename = "IdentificationID")]
    pub identification_id: Option<cct::Identifier>,
/// The date of this transport event.
    #[serde(default, rename = "OccurrenceDate")]
    pub occurrence_date: Option<udt::DateTime>,
/// The time of this transport event.
    #[serde(default, rename = "OccurrenceTime")]
    pub occurrence_time: Option<udt::DateTime>,
/// A code signifying the type of this transport event.
    #[serde(default, rename = "TransportEventTypeCode")]
    pub transport_event_type_code: Option<cct::Code>,
/// Text describing this transport event.
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
/// An indicator that this transport event has been completed (true) or not (false).
    #[serde(default, rename = "CompletionIndicator")]
    pub completion_indicator: Option<udt::Indicator>,
/// The shipment involved in this transport event.
    #[serde(default, rename = "ReportedShipment")]
    pub reported_shipment: Option<Box<crate::Shipment>>,
/// The current status of this transport event.
    #[serde(default, rename = "CurrentStatus")]
    pub current_status: Vec<crate::Status>,
/// The Party reponsible for this Transport Event.
    #[serde(default, rename = "ResponsibleParty")]
    pub responsible_party: Option<crate::Party>,
/// A contact associated with this transport event.
    #[serde(default, rename = "Contact")]
    pub contact: Vec<crate::Contact>,
/// The location associated with this transport event.
    #[serde(default, rename = "Location")]
    pub location: Option<crate::Location>,
/// A signature that can be used to sign for an entry or an exit at a transport location (e.g., port
/// terminal).
    #[serde(default, rename = "Signature")]
    pub signature: Option<crate::Signature>,
/// A period of time associated with this transport event.
    #[serde(default, rename = "Period")]
    pub period: Vec<crate::Period>,
}
