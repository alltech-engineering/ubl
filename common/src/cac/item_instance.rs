#[derive(Debug, Deserialize, Serialize)]
pub struct ItemInstance {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "ProductTraceID")]
    pub product_trace_id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "ManufactureDate")]
    pub manufacture_date: Option<super::udt::DateTimeType>,
    #[serde(default, rename = "ManufactureTime")]
    pub manufacture_time: Option<super::udt::DateTimeType>,
    #[serde(default, rename = "BestBeforeDate")]
    pub best_before_date: Option<super::udt::DateTimeType>,
    #[serde(default, rename = "RegistrationID")]
    pub registration_id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "SerialID")]
    pub serial_id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "AdditionalItemProperty")]
    pub additional_item_property: Vec<ItemProperty>,
    #[serde(default, rename = "LotIdentification")]
    pub lot_identification: Option<LotIdentification>,
}
