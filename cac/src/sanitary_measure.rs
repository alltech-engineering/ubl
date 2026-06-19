#[derive(Debug, Deserialize, Serialize)]
/// A class describing a plan, action or meassure that has been implemented for sanitary reasons.
///
/// UBL Dictionary Entry Name: `Sanitary Measure. Details`
///
/// Generated from XSD type `SanitaryMeasureType`.
pub struct SanitaryMeasure {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// A code decribing the type of sanitary meassure.
    #[serde(rename = "SanitaryMeasureTypeCode")]
    pub sanitary_measure_type_code: cct::Code,
/// The date this sanitary meassure was applied.
    #[serde(default, rename = "ApplicationDate")]
    pub application_date: Option<udt::DateTime>,
}
