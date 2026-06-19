#[derive(Debug, Deserialize, Serialize)]
/// A class to describe a specific, trackable instance of an item.
///
/// UBL Dictionary Entry Name: `Item Instance. Details`
///
/// Generated from XSD type `ItemInstanceType`.
pub struct ItemInstance {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// An identifier used for tracing this item instance, such as the EPC number used in RFID.
    #[serde(default, rename = "ProductTraceID")]
    pub product_trace_id: Option<cct::Identifier>,
/// The date on which this item instance was manufactured.
    #[serde(default, rename = "ManufactureDate")]
    pub manufacture_date: Option<udt::DateTime>,
/// The time at which this item instance was manufactured.
    #[serde(default, rename = "ManufactureTime")]
    pub manufacture_time: Option<udt::DateTime>,
/// The date before which it is best to use this item instance.
    #[serde(default, rename = "BestBeforeDate")]
    pub best_before_date: Option<udt::DateTime>,
/// The registration identifier of this item instance.
    #[serde(default, rename = "RegistrationID")]
    pub registration_id: Option<cct::Identifier>,
/// The serial number of this item instance.
    #[serde(default, rename = "SerialID")]
    pub serial_id: Option<cct::Identifier>,
/// An additional property of this item instance.
    #[serde(default, rename = "AdditionalItemProperty")]
    pub additional_item_property: Vec<ItemProperty>,
/// The lot identifier of this item instance (the identifier that allows recall of the item if
/// necessary).
    #[serde(default, rename = "LotIdentification")]
    pub lot_identification: Option<crate::LotIdentification>,
}
