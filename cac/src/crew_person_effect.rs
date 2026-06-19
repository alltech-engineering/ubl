#[derive(Debug, Deserialize, Serialize)]
/// A class describing the effect or belonging of a Crew Person
///
/// UBL Dictionary Entry Name: `Crew Person Effect. Details`
///
/// Generated from XSD type `CrewPersonEffectType`.
pub struct CrewPersonEffect {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// The description of the crew effect.
    #[serde(default, rename = "EffectDescription")]
    pub effect_description: Vec<cct::Text>,
/// The crew person to whom the effect belongs.
    #[serde(default, rename = "CrewPerson")]
    pub crew_person: Option<Person>,
}
