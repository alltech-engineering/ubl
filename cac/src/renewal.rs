#[derive(Debug, Deserialize, Serialize)]
pub struct Renewal {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
    #[serde(default, rename = "Amount")]
    pub amount: Option<cct::Amount>,
    #[serde(default, rename = "Period")]
    pub period: Option<Period>,
}
