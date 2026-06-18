#[derive(Debug, Deserialize, Serialize)]
pub struct DependentPriceReference {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "Percent")]
    pub percent: Option<super::cct::NumericType>,
    #[serde(default, rename = "LocationAddress")]
    pub location_address: Option<Address>,
    #[serde(default, rename = "DependentLineReference")]
    pub dependent_line_reference: Option<LineReference>,
}
