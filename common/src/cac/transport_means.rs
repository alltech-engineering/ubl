#[derive(Debug, Deserialize, Serialize)]
pub struct TransportMeans {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "JourneyID")]
    pub journey_id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "RegistrationNationalityID")]
    pub registration_nationality_id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "RegistrationNationality")]
    pub registration_nationality: Vec<super::cct::TextType>,
    #[serde(default, rename = "DirectionCode")]
    pub direction_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "TransportMeansTypeCode")]
    pub transport_means_type_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "TradeServiceCode")]
    pub trade_service_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "Stowage")]
    pub stowage: Option<Stowage>,
    #[serde(default, rename = "AirTransport")]
    pub air_transport: Option<AirTransport>,
    #[serde(default, rename = "RoadTransport")]
    pub road_transport: Option<RoadTransport>,
    #[serde(default, rename = "RailTransport")]
    pub rail_transport: Option<RailTransport>,
    #[serde(default, rename = "MaritimeTransport")]
    pub maritime_transport: Option<MaritimeTransport>,
    #[serde(default, rename = "OwnerParty")]
    pub owner_party: Option<Party>,
    #[serde(default, rename = "MeasurementDimension")]
    pub measurement_dimension: Vec<Dimension>,
}
