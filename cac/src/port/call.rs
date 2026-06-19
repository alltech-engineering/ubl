#[derive(Debug, Deserialize, Serialize)]
/// A class to describe a call to a port.
///
/// UBL Dictionary Entry Name: `Port Call. Details`
///
/// Generated from XSD type `PortCallType`.
pub struct PortCall {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// An identifier for this Port Call.
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
/// Description of the planned operations in this Port Call.
    #[serde(default, rename = "PlannedOperationsDescription")]
    pub planned_operations_description: Vec<cct::Text>,
/// Description of the planned works in this Port Call.
    #[serde(default, rename = "PlannedWorksDescription")]
    pub planned_works_description: Vec<cct::Text>,
/// Description of the planned inspections in this Port Call.
    #[serde(default, rename = "PlannedInspectionsDescription")]
    pub planned_inspections_description: Vec<cct::Text>,
/// An indicator of whether the ship is expected to stay at an anchorage upon arrival at the port of
/// call (true) or not (false).
    #[serde(default, rename = "ExpectedAnchorageIndicator")]
    pub expected_anchorage_indicator: Option<udt::Indicator>,
/// An identifier for the position in the port for this Port Call
    #[serde(default, rename = "PositionInPortID")]
    pub position_in_port_id: Option<cct::Identifier>,
/// Description about the condition of the cargo and ballast tank.
    #[serde(default, rename = "CargoAndBallastTankConditionDescription")]
    pub cargo_and_ballast_tank_condition_description: Vec<cct::Text>,
/// Ship requirements for this port call.
    #[serde(default, rename = "ShipRequirement")]
    pub ship_requirement: Vec<crate::ShipRequirement>,
/// The primary purpose of this port call.
    #[serde(default, rename = "PrimaryPortCallPurpose")]
    pub primary_port_call_purpose: Option<PortCallPurpose>,
/// Any additional or secondary purposes of this port call.
    #[serde(default, rename = "AdditionalPortCallPurpose")]
    pub additional_port_call_purpose: Vec<PortCallPurpose>,
/// The requested arrival event.
    #[serde(default, rename = "RequestedArrivalEvent")]
    pub requested_arrival_event: Option<crate::Event>,
}
