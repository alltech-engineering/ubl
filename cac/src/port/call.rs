#[derive(Debug, Deserialize, Serialize)]
pub struct PortCall {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
    #[serde(default, rename = "PlannedOperationsDescription")]
    pub planned_operations_description: Vec<cct::Text>,
    #[serde(default, rename = "PlannedWorksDescription")]
    pub planned_works_description: Vec<cct::Text>,
    #[serde(default, rename = "PlannedInspectionsDescription")]
    pub planned_inspections_description: Vec<cct::Text>,
    #[serde(default, rename = "ExpectedAnchorageIndicator")]
    pub expected_anchorage_indicator: Option<udt::Indicator>,
    #[serde(default, rename = "PositionInPortID")]
    pub position_in_port_id: Option<cct::Identifier>,
    #[serde(default, rename = "CargoAndBallastTankConditionDescription")]
    pub cargo_and_ballast_tank_condition_description: Vec<cct::Text>,
    #[serde(default, rename = "ShipRequirement")]
    pub ship_requirement: Vec<crate::ShipRequirement>,
    #[serde(default, rename = "PrimaryPortCallPurpose")]
    pub primary_port_call_purpose: Option<PortCallPurpose>,
    #[serde(default, rename = "AdditionalPortCallPurpose")]
    pub additional_port_call_purpose: Vec<PortCallPurpose>,
    #[serde(default, rename = "RequestedArrivalEvent")]
    pub requested_arrival_event: Option<crate::Event>,
}
