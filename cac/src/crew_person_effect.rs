#[derive(Debug, Deserialize, Serialize)]
pub struct CrewPersonEffect {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
    #[serde(default, rename = "EffectDescription")]
    pub effect_description: Vec<cct::Text>,
    #[serde(default, rename = "CrewPerson")]
    pub crew_person: Option<Person>,
}
