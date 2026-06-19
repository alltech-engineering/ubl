#[derive(Debug, Deserialize, Serialize)]
/// A class to describe a pickup for delivery.
///
/// UBL Dictionary Entry Name: `Pickup. Details`
///
/// Generated from XSD type `PickupType`.
pub struct Pickup {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// An identifier for this pickup.
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
/// The actual pickup date.
    #[serde(default, rename = "ActualPickupDate")]
    pub actual_pickup_date: Option<udt::DateTime>,
/// The actual pickup time.
    #[serde(default, rename = "ActualPickupTime")]
    pub actual_pickup_time: Option<udt::DateTime>,
/// The earliest pickup date.
    #[serde(default, rename = "EarliestPickupDate")]
    pub earliest_pickup_date: Option<udt::DateTime>,
/// The earliest pickup time.
    #[serde(default, rename = "EarliestPickupTime")]
    pub earliest_pickup_time: Option<udt::DateTime>,
/// The latest pickup date.
    #[serde(default, rename = "LatestPickupDate")]
    pub latest_pickup_date: Option<udt::DateTime>,
/// The latest pickup time.
    #[serde(default, rename = "LatestPickupTime")]
    pub latest_pickup_time: Option<udt::DateTime>,
/// The pickup location.
    #[serde(default, rename = "PickupLocation")]
    pub pickup_location: Option<Location>,
/// The Party who picks up the Delivery.
    #[serde(default, rename = "PickupParty")]
    pub pickup_party: Option<Party>,
}
