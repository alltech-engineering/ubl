#[derive(Debug, Deserialize, Serialize)]
pub struct ItemInstance {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "ProductTraceID")]
    pub product_trace_id: Option<cct::Identifier>,
    #[serde(default, rename = "ManufactureDate")]
    pub manufacture_date: Option<udt::DateTime>,
    #[serde(default, rename = "ManufactureTime")]
    pub manufacture_time: Option<udt::DateTime>,
    #[serde(default, rename = "BestBeforeDate")]
    pub best_before_date: Option<udt::DateTime>,
    #[serde(default, rename = "RegistrationID")]
    pub registration_id: Option<cct::Identifier>,
    #[serde(default, rename = "SerialID")]
    pub serial_id: Option<cct::Identifier>,
    #[serde(default, rename = "AdditionalItemProperty")]
    pub additional_item_property: Vec<ItemProperty>,
    #[serde(default, rename = "LotIdentification")]
    pub lot_identification: Option<crate::LotIdentification>,
}
