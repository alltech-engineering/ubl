#[derive(Debug, Deserialize, Serialize)]
pub struct MeterProperty {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "Name")]
    pub name: Option<super::cct::TextType>,
    #[serde(default, rename = "NameCode")]
    pub name_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "Value")]
    pub value: Option<super::cct::TextType>,
    #[serde(default, rename = "ValueQuantity")]
    pub value_quantity: Option<super::cct::QuantityType>,
    #[serde(default, rename = "ValueQualifier")]
    pub value_qualifier: Vec<super::cct::TextType>,
}
