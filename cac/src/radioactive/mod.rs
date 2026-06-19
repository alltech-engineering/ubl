use serde::{Deserialize, Serialize};


include!("isotope.rs");

#[derive(Debug, Deserialize, Serialize)]
pub struct RadioactiveMaterial {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
    #[serde(rename = "Name")]
    pub name: cct::Text,
    #[serde(default, rename = "SpecialFormDescription")]
    pub special_form_description: Vec<cct::Text>,
    #[serde(default, rename = "TransportIndexNumeric")]
    pub transport_index_numeric: Option<cct::Numeric>,
    #[serde(default, rename = "FissileCriticalitySafetyIndexNumeric")]
    pub fissile_criticality_safety_index_numeric: Option<cct::Numeric>,
    #[serde(default, rename = "ApplicableRadioactiveIsotope")]
    pub applicable_radioactive_isotope: Option<RadioactiveIsotope>,
}
