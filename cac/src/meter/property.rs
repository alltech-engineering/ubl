#[derive(Debug, Deserialize, Serialize)]
pub struct MeterProperty {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "Name")]
    pub name: Option<cct::Text>,
    #[serde(default, rename = "NameCode")]
    pub name_code: Option<cct::Code>,
    #[serde(default, rename = "Value")]
    pub value: Option<cct::Text>,
    #[serde(default, rename = "ValueQuantity")]
    pub value_quantity: Option<cct::Quantity>,
    #[serde(default, rename = "ValueQualifier")]
    pub value_qualifier: Vec<cct::Text>,
}
