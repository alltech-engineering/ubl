#[derive(Debug, Deserialize, Serialize)]
pub enum QualifierType {
    #[serde(rename = "OIDAsURI")]
    OidAsUri,
    #[serde(rename = "OIDAsURN")]
    OidAsUrn,
}
