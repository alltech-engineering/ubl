#[derive(Debug, Deserialize, Serialize)]
pub struct TransportSchedule {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(rename = "SequenceNumeric")]
    pub sequence_numeric: super::cct::NumericType,
    #[serde(default, rename = "ReferenceDate")]
    pub reference_date: Option<super::udt::DateTimeType>,
    #[serde(default, rename = "ReferenceTime")]
    pub reference_time: Option<super::udt::DateTimeType>,
    #[serde(default, rename = "ReliabilityPercent")]
    pub reliability_percent: Option<super::cct::NumericType>,
    #[serde(default, rename = "Remarks")]
    pub remarks: Vec<super::cct::TextType>,
    #[serde(rename = "StatusLocation")]
    pub status_location: Location,
    #[serde(default, rename = "ActualArrivalTransportEvent")]
    pub actual_arrival_transport_event: Option<TransportEvent>,
    #[serde(default, rename = "ActualDepartureTransportEvent")]
    pub actual_departure_transport_event: Option<TransportEvent>,
    #[serde(default, rename = "EstimatedDepartureTransportEvent")]
    pub estimated_departure_transport_event: Option<TransportEvent>,
    #[serde(default, rename = "EstimatedArrivalTransportEvent")]
    pub estimated_arrival_transport_event: Option<TransportEvent>,
    #[serde(default, rename = "PlannedDepartureTransportEvent")]
    pub planned_departure_transport_event: Option<TransportEvent>,
    #[serde(default, rename = "PlannedArrivalTransportEvent")]
    pub planned_arrival_transport_event: Option<TransportEvent>,
}
