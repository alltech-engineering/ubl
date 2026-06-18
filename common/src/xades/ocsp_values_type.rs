#[derive(Debug, Deserialize, Serialize)]
pub struct OcspValuesType {
    #[serde(default, rename = "EncapsulatedOCSPValue")]
    pub encapsulated_ocsp_value: Vec<EncapsulatedPkiData>,
}
