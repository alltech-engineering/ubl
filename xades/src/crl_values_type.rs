#[derive(Debug, Deserialize, Serialize)]
pub struct CrlValuesType {
    #[serde(default, rename = "EncapsulatedCRLValue")]
    pub encapsulated_crl_value: Vec<EncapsulatedPkiData>,
}
