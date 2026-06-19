#[derive(Debug, Deserialize, Serialize)]
pub enum X509DataTypeContent {
    #[serde(rename = "X509IssuerSerial")]
    X509IssuerSerial(X509IssuerSerialType),
    #[serde(rename = "X509SKI")]
    X509Ski(String),
    #[serde(rename = "X509SubjectName")]
    X509SubjectName(String),
    #[serde(rename = "X509Certificate")]
    X509Certificate(String),
    #[serde(rename = "X509CRL")]
    X509Crl(String),
    #[serde(rename = "any21")]
    Any(String),
}
