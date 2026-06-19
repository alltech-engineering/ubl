#[derive(Debug, Deserialize, Serialize)]
/// A class to describe a delivery unit.
///
/// UBL Dictionary Entry Name: `Delivery Unit. Details`
///
/// Generated from XSD type `DeliveryUnitType`.
pub struct DeliveryUnit {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// The quantity of ordered Items that constitutes a batch for delivery purposes.
    #[serde(rename = "BatchQuantity")]
    pub batch_quantity: cct::Quantity,
/// The quantity of units in the Delivery Unit expressed in the units used by the consumer.
    #[serde(default, rename = "ConsumerUnitQuantity")]
    pub consumer_unit_quantity: Option<cct::Quantity>,
/// An indication that the transported goods are subject to an international regulation concerning the
/// carriage of dangerous goods (true) or not (false).
    #[serde(default, rename = "HazardousRiskIndicator")]
    pub hazardous_risk_indicator: Option<udt::Indicator>,
}
