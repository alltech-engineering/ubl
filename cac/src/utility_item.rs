#[derive(Debug, Deserialize, Serialize)]
/// A class to describe the consumption of a utility product.
///
/// UBL Dictionary Entry Name: `Utility Item. Details`
///
/// Generated from XSD type `UtilityItemType`.
pub struct UtilityItem {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// An identifier for this utility item.
    #[serde(rename = "ID")]
    pub id: cct::Identifier,
/// An identifier for the subscriber to the utility.
    #[serde(default, rename = "SubscriberID")]
    pub subscriber_id: Option<cct::Identifier>,
/// Identification of the subscriber type, expressed as text..
    #[serde(default, rename = "SubscriberType")]
    pub subscriber_type: Option<cct::Text>,
/// The code identifying for the service type.
    #[serde(default, rename = "SubscriberTypeCode")]
    pub subscriber_type_code: Option<cct::Code>,
/// Text describing the consumption product.
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
/// The unit packaging quantity.
    #[serde(default, rename = "PackQuantity")]
    pub pack_quantity: Option<cct::Quantity>,
/// The number of items in a pack.
    #[serde(default, rename = "PackSizeNumeric")]
    pub pack_size_numeric: Option<cct::Numeric>,
/// The type of product consumed, expressed as text.
    #[serde(default, rename = "ConsumptionType")]
    pub consumption_type: Option<cct::Text>,
/// The type of product consumed, expressed as a code.
    #[serde(default, rename = "ConsumptionTypeCode")]
    pub consumption_type_code: Option<cct::Code>,
/// Information of the actual payments type for the utility Item
    #[serde(default, rename = "CurrentChargeType")]
    pub current_charge_type: Option<cct::Text>,
/// Information of the actual payments type code expressed as a code
    #[serde(default, rename = "CurrentChargeTypeCode")]
    pub current_charge_type_code: Option<cct::Code>,
/// Information about the one-time payment type in case everything is paid One time
    #[serde(default, rename = "OneTimeChargeType")]
    pub one_time_charge_type: Option<cct::Text>,
/// Information about the one-time payment type code
    #[serde(default, rename = "OneTimeChargeTypeCode")]
    pub one_time_charge_type_code: Option<cct::Code>,
/// The tax category applicable to this utility item.
    #[serde(default, rename = "TaxCategory")]
    pub tax_category: Option<TaxCategory>,
/// A contract setting forth conditions applicable to this utility item.
    #[serde(default, rename = "Contract")]
    pub contract: Option<Contract>,
}
