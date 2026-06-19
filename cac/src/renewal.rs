#[derive(Debug, Deserialize, Serialize)]
pub struct Renewal {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "Amount")]
    pub amount: Option<cct::Amount>,
    #[serde(default, rename = "Period")]
    pub period: Option<Period>,
}
