#[derive(Debug, Deserialize, Serialize)]
pub struct Delivery {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "Quantity")]
    pub quantity: Option<super::cct::QuantityType>,
    #[serde(default, rename = "MinimumQuantity")]
    pub minimum_quantity: Option<super::cct::QuantityType>,
    #[serde(default, rename = "MaximumQuantity")]
    pub maximum_quantity: Option<super::cct::QuantityType>,
    #[serde(default, rename = "ActualDeliveryDate")]
    pub actual_delivery_date: Option<super::udt::DateTimeType>,
    #[serde(default, rename = "ActualDeliveryTime")]
    pub actual_delivery_time: Option<super::udt::DateTimeType>,
    #[serde(default, rename = "LatestDeliveryDate")]
    pub latest_delivery_date: Option<super::udt::DateTimeType>,
    #[serde(default, rename = "LatestDeliveryTime")]
    pub latest_delivery_time: Option<super::udt::DateTimeType>,
    #[serde(default, rename = "ReleaseID")]
    pub release_id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "TrackingID")]
    pub tracking_id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "DeliveryAddress")]
    pub delivery_address: Option<Address>,
    #[serde(default, rename = "DeliveryLocation")]
    pub delivery_location: Option<Location>,
    #[serde(default, rename = "AlternativeDeliveryLocation")]
    pub alternative_delivery_location: Option<Location>,
    #[serde(default, rename = "RequestedDeliveryPeriod")]
    pub requested_delivery_period: Option<Period>,
    #[serde(default, rename = "PromisedDeliveryPeriod")]
    pub promised_delivery_period: Option<Period>,
    #[serde(default, rename = "EstimatedDeliveryPeriod")]
    pub estimated_delivery_period: Option<Period>,
    #[serde(default, rename = "CarrierParty")]
    pub carrier_party: Option<Party>,
    #[serde(default, rename = "DeliveryParty")]
    pub delivery_party: Option<Party>,
    #[serde(default, rename = "NotifyParty")]
    pub notify_party: Vec<Party>,
    #[serde(default, rename = "Despatch")]
    pub despatch: Option<Despatch>,
    #[serde(default, rename = "DeliveryTerms")]
    pub delivery_terms: Vec<DeliveryTerms>,
    #[serde(default, rename = "MinimumDeliveryUnit")]
    pub minimum_delivery_unit: Option<DeliveryUnit>,
    #[serde(default, rename = "MaximumDeliveryUnit")]
    pub maximum_delivery_unit: Option<DeliveryUnit>,
    #[serde(default, rename = "Shipment")]
    pub shipment: Option<Shipment>,
    #[serde(default, rename = "FuelConsumption")]
    pub fuel_consumption: Vec<FuelConsumption>,
    #[serde(default, rename = "DeliveryNoteDocumentReference")]
    pub delivery_note_document_reference: Vec<DocumentReference>,
    #[serde(default, rename = "DeliveryNoteLineReference")]
    pub delivery_note_line_reference: Vec<LineReference>,
}
