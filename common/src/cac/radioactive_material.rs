#[derive(Debug, Deserialize, Serialize)]
pub struct RadioactiveMaterial {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(rename = "Name")]
    pub name: super::cct::TextType,
    #[serde(default, rename = "SpecialFormDescription")]
    pub special_form_description: Vec<super::cct::TextType>,
    #[serde(default, rename = "TransportIndexNumeric")]
    pub transport_index_numeric: Option<super::cct::NumericType>,
    #[serde(default, rename = "FissileCriticalitySafetyIndexNumeric")]
    pub fissile_criticality_safety_index_numeric: Option<super::cct::NumericType>,
    #[serde(default, rename = "ApplicableRadioactiveIsotope")]
    pub applicable_radioactive_isotope: Option<RadioactiveIsotope>,
}
