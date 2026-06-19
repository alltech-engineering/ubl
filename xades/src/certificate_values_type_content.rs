#[derive(Debug, Deserialize, Serialize)]
pub enum CertificateValuesTypeContent {
    #[serde(rename = "EncapsulatedX509Certificate")]
    EncapsulatedX509Certificate(EncapsulatedPkiData),
    #[serde(rename = "OtherCertificate")]
    OtherCertificate(Any),
}
