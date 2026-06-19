#[derive(Debug, Deserialize, Serialize)]
/// A class for defining a set of geographical coordinates (apparently misnamed).
///
/// UBL Dictionary Entry Name: `Location Coordinate. Details`
///
/// Generated from XSD type `LocationCoordinateType`.
pub struct LocationCoordinate {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
/// A code signifying the location system used.
    #[serde(default, rename = "CoordinateSystemCode")]
    pub coordinate_system_code: Option<cct::Code>,
/// The degree component of a latitude measured in degrees and minutes.
    #[serde(default, rename = "LatitudeDegreesMeasure")]
    pub latitude_degrees_measure: Option<cct::Measure>,
/// The minutes component of a latitude measured in degrees and minutes (modulo 60).
    #[serde(default, rename = "LatitudeMinutesMeasure")]
    pub latitude_minutes_measure: Option<cct::Measure>,
/// A code signifying the direction of latitude measurement from the equator (north or south).
    #[serde(default, rename = "LatitudeDirectionCode")]
    pub latitude_direction_code: Option<cct::Code>,
/// The degree component of a longitude measured in degrees and minutes.
    #[serde(default, rename = "LongitudeDegreesMeasure")]
    pub longitude_degrees_measure: Option<cct::Measure>,
/// The minutes component of a longitude measured in degrees and minutes (modulo 60).
    #[serde(default, rename = "LongitudeMinutesMeasure")]
    pub longitude_minutes_measure: Option<cct::Measure>,
/// A code signifying the direction of longitude measurement from the prime meridian (east or west).
    #[serde(default, rename = "LongitudeDirectionCode")]
    pub longitude_direction_code: Option<cct::Code>,
/// The altitude of the location.
    #[serde(default, rename = "AltitudeMeasure")]
    pub altitude_measure: Option<cct::Measure>,
}
