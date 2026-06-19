#[derive(Debug, Deserialize, Serialize)]
pub struct RailTransport {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(rename = "TrainID")]
    pub train_id: cct::Identifier,
    #[serde(default, rename = "RailCarID")]
    pub rail_car_id: Option<cct::Identifier>,
}
