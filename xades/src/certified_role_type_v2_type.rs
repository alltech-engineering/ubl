#[derive(Debug, Deserialize, Serialize)]
pub enum CertifiedRoleTypeV2Type {
    #[serde(rename = "X509AttributeCertificate")]
    X509AttributeCertificate(EncapsulatedPkiData),
    #[serde(rename = "OtherAttributeCertificate")]
    OtherAttributeCertificate(Any),
}
