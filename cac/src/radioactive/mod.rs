use serde::{Deserialize, Serialize};


include!("isotope.rs");

#[derive(Debug, Deserialize, Serialize)]
/// A class defining a radioactive material.
///
/// UBL Dictionary Entry Name: `Radioactive Material. Details`
///
/// Generated from XSD type `RadioactiveMaterialType`.
pub struct RadioactiveMaterial {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
/// The name of this Iradioactive Material.
    #[serde(rename = "Name")]
    pub name: cct::Text,
/// The description of the physical and chemical form of the material, including any notation that the
/// material is a special form radioactive material or a low dispersible radioactive material.
    #[serde(default, rename = "SpecialFormDescription")]
    pub special_form_description: Vec<cct::Text>,
/// A number specifying the Transport Index for this Radioactive Material.
    #[serde(default, rename = "TransportIndexNumeric")]
    pub transport_index_numeric: Option<cct::Numeric>,
/// The number assigned to and placed on the label of a fissile radioactive material package to
/// designate the degree of control of accumulation of packages, overpacks or freight containers
/// containing fissile material during transportation.
    #[serde(default, rename = "FissileCriticalitySafetyIndexNumeric")]
    pub fissile_criticality_safety_index_numeric: Option<cct::Numeric>,
/// The maximum activity of the radioactive contents capable of sustaining a nuclear fission chain
/// reaction during carriage
    #[serde(default, rename = "ApplicableRadioactiveIsotope")]
    pub applicable_radioactive_isotope: Option<RadioactiveIsotope>,
}
