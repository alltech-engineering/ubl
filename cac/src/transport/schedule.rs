#[derive(Debug, Deserialize, Serialize)]
/// Describes the location and schedule relating to a transport means.
///
/// UBL Dictionary Entry Name: `Transport Schedule. Details`
///
/// Generated from XSD type `TransportScheduleType`.
pub struct TransportSchedule {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
/// A number indicating the order of this status in the sequence in which statuses are to be presented.
    #[serde(rename = "SequenceNumeric")]
    pub sequence_numeric: cct::Numeric,
/// The reference date for the transport schedule status.
    #[serde(default, rename = "ReferenceDate")]
    pub reference_date: Option<udt::DateTime>,
/// The reference time for the transport schedule status.
    #[serde(default, rename = "ReferenceTime")]
    pub reference_time: Option<udt::DateTime>,
/// The reliability of the transport schedule status, expressed as a percentage.
    #[serde(default, rename = "ReliabilityPercent")]
    pub reliability_percent: Option<cct::Numeric>,
/// Remarks related to the transport schedule status.
    #[serde(default, rename = "Remarks")]
    pub remarks: Vec<cct::Text>,
/// The location for which status is reported.
    #[serde(rename = "StatusLocation")]
    pub status_location: crate::Location,
/// The actual arrival at a location.
    #[serde(default, rename = "ActualArrivalTransportEvent")]
    pub actual_arrival_transport_event: Option<TransportEvent>,
/// The actual departure from a location.
    #[serde(default, rename = "ActualDepartureTransportEvent")]
    pub actual_departure_transport_event: Option<TransportEvent>,
/// An estimated departure from a specified location.
    #[serde(default, rename = "EstimatedDepartureTransportEvent")]
    pub estimated_departure_transport_event: Option<TransportEvent>,
/// An estimated arrival at a specified location.
    #[serde(default, rename = "EstimatedArrivalTransportEvent")]
    pub estimated_arrival_transport_event: Option<TransportEvent>,
/// The planned departure from a specified location.
    #[serde(default, rename = "PlannedDepartureTransportEvent")]
    pub planned_departure_transport_event: Option<TransportEvent>,
/// The planned arrival at a specified location.
    #[serde(default, rename = "PlannedArrivalTransportEvent")]
    pub planned_arrival_transport_event: Option<TransportEvent>,
}
