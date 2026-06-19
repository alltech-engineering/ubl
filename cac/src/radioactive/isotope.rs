#[derive(Debug, Deserialize, Serialize)]
/// A class defining the maximum activity of the radioactive contents capable of sustaining a nuclear
/// fission chain reaction during carriage.
///
/// UBL Dictionary Entry Name: `Radioactive Isotope. Details`
///
/// Generated from XSD type `RadioactiveIsotopeType`.
pub struct RadioactiveIsotope {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
/// The name of this Iradioactive Isotope.
    #[serde(rename = "Name")]
    pub name: cct::Text,
/// The measure of the radioactive contents during carriage.
    #[serde(rename = "ActivityLevelMeasure")]
    pub activity_level_measure: cct::Measure,
}
