#[derive(Debug, Deserialize, Serialize)]
/// The name of this meter property.
///
/// UBL Dictionary Entry Name: `Meter Property. Details`
///
/// Generated from XSD type `MeterPropertyType`.
pub struct MeterProperty {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// The name of this meter property, expressed as a code.
    #[serde(default, rename = "Name")]
    pub name: Option<cct::Text>,
/// The value of this meter property, expressed as text.
    #[serde(default, rename = "NameCode")]
    pub name_code: Option<cct::Code>,
/// The value of this meter property, expressed as a quantity.
    #[serde(default, rename = "Value")]
    pub value: Option<cct::Text>,
/// The value of this meter property, expressed as a quantity.
    #[serde(default, rename = "ValueQuantity")]
    pub value_quantity: Option<cct::Quantity>,
/// An additional value to qualify the value of the meter
    #[serde(default, rename = "ValueQualifier")]
    pub value_qualifier: Vec<cct::Text>,
}
