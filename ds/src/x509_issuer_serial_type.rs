#[derive(Debug, Deserialize, Serialize)]
pub struct X509IssuerSerialType {
    #[serde(rename = "X509IssuerName")]
    pub x509_issuer_name: String,
    #[serde(rename = "X509SerialNumber")]
    pub x509_serial_number: i32,
}
