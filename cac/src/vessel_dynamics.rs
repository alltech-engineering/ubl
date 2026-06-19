#[derive(Debug, Deserialize, Serialize)]
/// A class to describe the dynamics of a vesssel.
///
/// UBL Dictionary Entry Name: `Vessel Dynamics. Details`
///
/// Generated from XSD type `VesselDynamicsType`.
pub struct VesselDynamics {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// A code specifying the navigation status for the vessel.
    #[serde(default, rename = "NavigationStatusCode")]
    pub navigation_status_code: Option<cct::Code>,
/// Indicates whether the vessel is at anchor.
    #[serde(default, rename = "AtAnchorageIndicator")]
    pub at_anchorage_indicator: Option<udt::Indicator>,
/// Text describing the actual direction of progress of a vessel, between two points, in relation to the
/// surface of the earth.
    #[serde(default, rename = "CourseOverGroundDirection")]
    pub course_over_ground_direction: Option<cct::Text>,
/// Text describing the speed of the vessel relative to the surface of the earth.
    #[serde(default, rename = "SpeedOverGroundMeasure")]
    pub speed_over_ground_measure: Option<cct::Measure>,
/// Text describing the rate at which the vessel is turning.
    #[serde(default, rename = "RateOfTurnMeasure")]
    pub rate_of_turn_measure: Option<cct::Measure>,
}
