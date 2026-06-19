#[derive(Debug, Deserialize, Serialize)]
pub struct VesselDynamics {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
    #[serde(default, rename = "NavigationStatusCode")]
    pub navigation_status_code: Option<cct::Code>,
    #[serde(default, rename = "AtAnchorageIndicator")]
    pub at_anchorage_indicator: Option<udt::Indicator>,
    #[serde(default, rename = "CourseOverGroundDirection")]
    pub course_over_ground_direction: Option<cct::Text>,
    #[serde(default, rename = "SpeedOverGroundMeasure")]
    pub speed_over_ground_measure: Option<cct::Measure>,
    #[serde(default, rename = "RateOfTurnMeasure")]
    pub rate_of_turn_measure: Option<cct::Measure>,
}
