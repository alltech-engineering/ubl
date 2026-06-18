#[derive(Debug, Deserialize, Serialize)]
pub struct LocationCoordinate {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "CoordinateSystemCode")]
    pub coordinate_system_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "LatitudeDegreesMeasure")]
    pub latitude_degrees_measure: Option<super::cct::MeasureType>,
    #[serde(default, rename = "LatitudeMinutesMeasure")]
    pub latitude_minutes_measure: Option<super::cct::MeasureType>,
    #[serde(default, rename = "LatitudeDirectionCode")]
    pub latitude_direction_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "LongitudeDegreesMeasure")]
    pub longitude_degrees_measure: Option<super::cct::MeasureType>,
    #[serde(default, rename = "LongitudeMinutesMeasure")]
    pub longitude_minutes_measure: Option<super::cct::MeasureType>,
    #[serde(default, rename = "LongitudeDirectionCode")]
    pub longitude_direction_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "AltitudeMeasure")]
    pub altitude_measure: Option<super::cct::MeasureType>,
}
