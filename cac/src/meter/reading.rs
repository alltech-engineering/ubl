#[derive(Debug, Deserialize, Serialize)]
/// A class to describe a meter reading.
///
/// UBL Dictionary Entry Name: `Meter Reading. Details`
///
/// Generated from XSD type `MeterReadingType`.
pub struct MeterReading {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// An identifier for this meter reading.
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
/// The type of this meter reading, expressed as text.
    #[serde(default, rename = "MeterReadingType")]
    pub meter_reading_type: Option<cct::Text>,
/// The type of this meter reading, expressed as a code.
    #[serde(default, rename = "MeterReadingTypeCode")]
    pub meter_reading_type_code: Option<cct::Code>,
/// The date of the previous meter reading.
    #[serde(rename = "PreviousMeterReadingDate")]
    pub previous_meter_reading_date: udt::DateTime,
/// The quantity of the previous meter reading.
    #[serde(rename = "PreviousMeterQuantity")]
    pub previous_meter_quantity: cct::Quantity,
/// The date of the latest meter reading.
    #[serde(rename = "LatestMeterReadingDate")]
    pub latest_meter_reading_date: udt::DateTime,
/// The quantity of the latest meter reading.
    #[serde(rename = "LatestMeterQuantity")]
    pub latest_meter_quantity: cct::Quantity,
/// The method used for the previous meter reading, expressed as text.
    #[serde(default, rename = "PreviousMeterReadingMethod")]
    pub previous_meter_reading_method: Option<cct::Text>,
/// The method used for the previous meter reading, expressed as a code.
    #[serde(default, rename = "PreviousMeterReadingMethodCode")]
    pub previous_meter_reading_method_code: Option<cct::Code>,
/// The method used for the latest meter reading, expressed as text.
    #[serde(default, rename = "LatestMeterReadingMethod")]
    pub latest_meter_reading_method: Option<cct::Text>,
/// The method used for the latest meter reading, expressed as a code.
    #[serde(default, rename = "LatestMeterReadingMethodCode")]
    pub latest_meter_reading_method_code: Option<cct::Code>,
/// Text containing comments on this meter reading.
    #[serde(default, rename = "MeterReadingComments")]
    pub meter_reading_comments: Vec<cct::Text>,
/// Consumption in the period from PreviousMeterReadingDate to LatestMeterReadingDate.
    #[serde(rename = "DeliveredQuantity")]
    pub delivered_quantity: cct::Quantity,
}
