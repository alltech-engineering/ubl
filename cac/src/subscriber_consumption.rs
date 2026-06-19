#[derive(Debug, Deserialize, Serialize)]
/// The consumption for a specific party for given consumption point provided by a numbers of suppliers.
/// An enterprise can have one utility statement for several parties (e.g. a ministry of defence
/// receiving a telephone bill). In this way each subscriber consumption represent a sub utility
/// statement.
///
/// UBL Dictionary Entry Name: `Subscriber Consumption. Details`
///
/// Generated from XSD type `SubscriberConsumptionType`.
pub struct SubscriberConsumption {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// The identifier tor this specification.
    #[serde(default, rename = "ConsumptionID")]
    pub consumption_id: Option<cct::Identifier>,
/// The code which specifies the type of this specification, e.g. an on account specification or the
/// yearly specification.
    #[serde(default, rename = "SpecificationTypeCode")]
    pub specification_type_code: Option<cct::Code>,
/// Free-form text conveying information that is not contained explicitly in other structures.
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
/// The total quantity consumed, as calculated from meter readings.
    #[serde(default, rename = "TotalMeteredQuantity")]
    pub total_metered_quantity: Option<cct::Quantity>,
/// The Party who is subscribed to the utility.
    #[serde(default, rename = "SubscriberParty")]
    pub subscriber_party: Option<Party>,
/// The point at which the utility is consumed.
    #[serde(rename = "UtilityConsumptionPoint")]
    pub utility_consumption_point: ConsumptionPoint,
/// The planned prepayments (on account) regarding this subscription.
    #[serde(default, rename = "OnAccountPayment")]
    pub on_account_payment: Vec<OnAccountPayment>,
/// The consumption in case the consumption is from one and only one supplier.
    #[serde(default, rename = "Consumption")]
    pub consumption: Option<Consumption>,
/// The consumption in case the consumption is from more than one supplier.
    #[serde(default, rename = "SupplierConsumption")]
    pub supplier_consumption: Vec<SupplierConsumption>,
}
