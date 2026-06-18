#[derive(Debug, Deserialize, Serialize)]
pub struct CrewPersonEffect {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "EffectDescription")]
    pub effect_description: Vec<super::cct::TextType>,
    #[serde(default, rename = "CrewPerson")]
    pub crew_person: Option<Person>,
}
