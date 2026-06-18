#[derive(Debug, Deserialize, Serialize)]
pub struct PortCall {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "PlannedOperationsDescription")]
    pub planned_operations_description: Vec<super::cct::TextType>,
    #[serde(default, rename = "PlannedWorksDescription")]
    pub planned_works_description: Vec<super::cct::TextType>,
    #[serde(default, rename = "PlannedInspectionsDescription")]
    pub planned_inspections_description: Vec<super::cct::TextType>,
    #[serde(default, rename = "ExpectedAnchorageIndicator")]
    pub expected_anchorage_indicator: Option<super::udt::IndicatorType>,
    #[serde(default, rename = "PositionInPortID")]
    pub position_in_port_id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "CargoAndBallastTankConditionDescription")]
    pub cargo_and_ballast_tank_condition_description: Vec<super::cct::TextType>,
    #[serde(default, rename = "ShipRequirement")]
    pub ship_requirement: Vec<ShipRequirement>,
    #[serde(default, rename = "PrimaryPortCallPurpose")]
    pub primary_port_call_purpose: Option<PortCallPurpose>,
    #[serde(default, rename = "AdditionalPortCallPurpose")]
    pub additional_port_call_purpose: Vec<PortCallPurpose>,
    #[serde(default, rename = "RequestedArrivalEvent")]
    pub requested_arrival_event: Option<Event>,
}
