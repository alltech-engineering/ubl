#[derive(Debug, Deserialize, Serialize)]
pub struct Pickup {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "ActualPickupDate")]
    pub actual_pickup_date: Option<super::udt::DateTimeType>,
    #[serde(default, rename = "ActualPickupTime")]
    pub actual_pickup_time: Option<super::udt::DateTimeType>,
    #[serde(default, rename = "EarliestPickupDate")]
    pub earliest_pickup_date: Option<super::udt::DateTimeType>,
    #[serde(default, rename = "EarliestPickupTime")]
    pub earliest_pickup_time: Option<super::udt::DateTimeType>,
    #[serde(default, rename = "LatestPickupDate")]
    pub latest_pickup_date: Option<super::udt::DateTimeType>,
    #[serde(default, rename = "LatestPickupTime")]
    pub latest_pickup_time: Option<super::udt::DateTimeType>,
    #[serde(default, rename = "PickupLocation")]
    pub pickup_location: Option<Location>,
    #[serde(default, rename = "PickupParty")]
    pub pickup_party: Option<Party>,
}
