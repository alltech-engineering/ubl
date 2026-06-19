use serde::{Deserialize, Serialize};


include!("reading.rs");
include!("property.rs");

#[derive(Debug, Deserialize, Serialize)]
/// A class to describe a meter and its readings.
///
/// UBL Dictionary Entry Name: `Meter. Details`
///
/// Generated from XSD type `MeterType`.
pub struct Meter {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// The meter number, expressed as text.
    #[serde(default, rename = "MeterNumber")]
    pub meter_number: Option<cct::Text>,
/// The name of this meter, which serves as an identifier to distinguish a main meter from a submeter.
    #[serde(default, rename = "MeterName")]
    pub meter_name: Option<cct::Text>,
/// The factor by which readings of this meter must be multiplied to calculate consumption, expressed as
/// text.
    #[serde(default, rename = "MeterConstant")]
    pub meter_constant: Option<cct::Text>,
/// A code signifying the formula to be used in applying the meter constant.
    #[serde(default, rename = "MeterConstantCode")]
    pub meter_constant_code: Option<cct::Code>,
/// The quantity delivered; the total quantity consumed as calculated from the meter readings.
    #[serde(default, rename = "TotalDeliveredQuantity")]
    pub total_delivered_quantity: Option<cct::Quantity>,
/// A reading of this meter.
    #[serde(default, rename = "MeterReading")]
    pub meter_reading: Vec<MeterReading>,
/// A property of this meter.
    #[serde(default, rename = "MeterProperty")]
    pub meter_property: Vec<MeterProperty>,
}
