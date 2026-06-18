#[derive(Debug, Deserialize, Serialize)]
pub struct RailTransport {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(rename = "TrainID")]
    pub train_id: super::cct::IdentifierType,
    #[serde(default, rename = "RailCarID")]
    pub rail_car_id: Option<super::cct::IdentifierType>,
}
