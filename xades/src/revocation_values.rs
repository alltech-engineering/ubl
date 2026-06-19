#[derive(Debug, Deserialize, Serialize)]
pub struct RevocationValues {
    #[serde(default, rename = "@Id")]
    pub id: Option<String>,
    #[serde(default, rename = "CRLValues")]
    pub crl_values: Option<CrlValuesType>,
    #[serde(default, rename = "OCSPValues")]
    pub ocsp_values: Option<OcspValuesType>,
    #[serde(default, rename = "OtherValues")]
    pub other_values: Option<OtherCertStatusValuesType>,
}
