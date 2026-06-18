#[derive(Debug, Deserialize, Serialize)]
pub struct SignatureMethod {
    #[serde(rename = "@Algorithm")]
    pub algorithm: String,
    #[serde(default, rename = "$text")]
    pub text_before: Option<String>,
    #[serde(default, rename = "HMACOutputLength")]
    pub hmac_output_length: Option<i32>,
    #[serde(default, rename = "any7")]
    pub any: Vec<String>,
}
