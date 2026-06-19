#[derive(Debug, Deserialize, Serialize)]
/// A class to describe the type and amount of resources consumed during a product’s lifecycle,
/// including information about source and timing.
///
/// UBL Dictionary Entry Name: `Resource Consumption. Details`
///
/// Generated from XSD type `ResourceConsumptionType`.
pub struct ResourceConsumption {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// A code identifying the type of resource (e.g., water, electricity, raw material).
    #[serde(rename = "ResourceTypeCode")]
    pub resource_type_code: cct::Code,
/// The amount of the resource consumed.
    #[serde(rename = "ConsumptionMeasure")]
    pub consumption_measure: cct::Measure,
/// A text description of the source or origin of the consumed resource.
    #[serde(default, rename = "ResourceOriginDescription")]
    pub resource_origin_description: Vec<cct::Text>,
/// The period during which this resource consumption was measured.
    #[serde(default, rename = "MeasurementPeriod")]
    pub measurement_period: Option<Period>,
}
