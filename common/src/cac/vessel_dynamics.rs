#[derive(Debug, Deserialize, Serialize)]
pub struct VesselDynamics {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "NavigationStatusCode")]
    pub navigation_status_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "AtAnchorageIndicator")]
    pub at_anchorage_indicator: Option<super::udt::IndicatorType>,
    #[serde(default, rename = "CourseOverGroundDirection")]
    pub course_over_ground_direction: Option<super::cct::TextType>,
    #[serde(default, rename = "SpeedOverGroundMeasure")]
    pub speed_over_ground_measure: Option<super::cct::MeasureType>,
    #[serde(default, rename = "RateOfTurnMeasure")]
    pub rate_of_turn_measure: Option<super::cct::MeasureType>,
}
