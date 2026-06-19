use serde::{Deserialize, Serialize};

pub type ItemNotificationParty = crate::Party;
pub type ItemPassportAttachment = crate::Attachment;

include!("information_request_line.rs");
include!("comparison.rs");
include!("identification.rs");
include!("instance.rs");
include!("property.rs");
include!("property_range.rs");
include!("management_profile.rs");
include!("location_quantity.rs");
include!("property_group.rs");

#[derive(Debug, Deserialize, Serialize)]
/// A class to describe an item of trade. It includes a generic description applicable to all examples
/// of the item together with optional subsidiary descriptions of any number of actual instances of the
/// type.
///
/// UBL Dictionary Entry Name: `Item. Details`
///
/// Generated from XSD type `ItemType`.
pub struct Item {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// Text describing this item.
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
/// The unit packaging quantity; the number of subunits making up this item.
    #[serde(default, rename = "PackQuantity")]
    pub pack_quantity: Option<cct::Quantity>,
/// The number of items in a pack of this item.
    #[serde(default, rename = "PackSizeNumeric")]
    pub pack_size_numeric: Option<cct::Numeric>,
/// An indicator that this item was ordered from a catalogue (true) or not (false).
    #[serde(default, rename = "CatalogueIndicator")]
    pub catalogue_indicator: Option<udt::Indicator>,
/// A short name optionally given to this item, such as a name from a catalogue, as distinct from a
/// description.
    #[serde(default, rename = "Name")]
    pub name: Vec<cct::Text>,
/// A code indicating the type of the item (eg., service, product, etc.).
    #[serde(default, rename = "ItemTypeCode")]
    pub item_type_code: Option<cct::Code>,
/// An indication that the transported item, as delivered, is subject to an international regulation
/// concerning the carriage of dangerous goods (true) or not (false).
    #[serde(default, rename = "HazardousRiskIndicator")]
    pub hazardous_risk_indicator: Option<udt::Indicator>,
/// Further details regarding this item (e.g., the URL of a relevant web page).
    #[serde(default, rename = "AdditionalInformation")]
    pub additional_information: Vec<cct::Text>,
/// A keyword (search string) for this item, assigned by the seller party. Can also be a synonym for the
/// name of the item.
    #[serde(default, rename = "Keyword")]
    pub keyword: Vec<cct::Text>,
/// A brand name of this item.
    #[serde(default, rename = "BrandName")]
    pub brand_name: Vec<cct::Text>,
/// A model name of this item.
    #[serde(default, rename = "ModelName")]
    pub model_name: Vec<cct::Text>,
/// Text describing the warranty for this item.
    #[serde(default, rename = "WarrantyInformation")]
    pub warranty_information: Vec<cct::Text>,
/// A code indicating the product’s lifecycle stage (e.g., sourcing, manufacturing, distribution, usage,
/// end-of-life)
    #[serde(default, rename = "LifecycleStageCode")]
    pub lifecycle_stage_code: Option<cct::Code>,
/// A text describing the specific environmental impact associated with this product's lifecycle stage.
    #[serde(default, rename = "LifecycleStageDescription")]
    pub lifecycle_stage_description: Vec<cct::Text>,
/// Identifying information for this item, assigned by the buyer.
    #[serde(default, rename = "BuyersItemIdentification")]
    pub buyers_item_identification: Option<ItemIdentification>,
/// Identifying information for this item, assigned by the seller.
    #[serde(default, rename = "SellersItemIdentification")]
    pub sellers_item_identification: Option<ItemIdentification>,
/// Identifying information for this item, assigned by the manufacturer.
    #[serde(default, rename = "ManufacturersItemIdentification")]
    pub manufacturers_item_identification: Vec<ItemIdentification>,
/// Identifying information for this item, assigned according to a standard system.
    #[serde(default, rename = "StandardItemIdentification")]
    pub standard_item_identification: Option<ItemIdentification>,
/// Identifying information for this item, assigned according to a cataloguing system.
    #[serde(default, rename = "CatalogueItemIdentification")]
    pub catalogue_item_identification: Option<ItemIdentification>,
/// An additional identifier for this item.
    #[serde(default, rename = "AdditionalItemIdentification")]
    pub additional_item_identification: Vec<ItemIdentification>,
/// A reference to the catalogue in which this item appears.
    #[serde(default, rename = "CatalogueDocumentReference")]
    pub catalogue_document_reference: Option<crate::DocumentReference>,
/// A reference to a specification document for this item.
    #[serde(default, rename = "ItemSpecificationDocumentReference")]
    pub item_specification_document_reference: Vec<crate::DocumentReference>,
/// The country of origin of this item.
    #[serde(default, rename = "OriginCountry")]
    pub origin_country: Option<crate::Country>,
/// A classification of this item according to a specific system for classifying commodities.
    #[serde(default, rename = "CommodityClassification")]
    pub commodity_classification: Vec<crate::CommodityClassification>,
/// A set of sales conditions applying to this item.
    #[serde(default, rename = "TransactionConditions")]
    pub transaction_conditions: Vec<crate::TransactionConditions>,
/// Information pertaining to this item as a hazardous item.
    #[serde(default, rename = "HazardousItem")]
    pub hazardous_item: Vec<crate::HazardousItem>,
/// A tax category applicable to this item.
    #[serde(default, rename = "ClassifiedTaxCategory")]
    pub classified_tax_category: Vec<crate::TaxCategory>,
/// An additional property of this item.
    #[serde(default, rename = "AdditionalItemProperty")]
    pub additional_item_property: Vec<ItemProperty>,
/// The Party who manufacters this Item.
    #[serde(default, rename = "ManufacturerParty")]
    pub manufacturer_party: Vec<crate::Party>,
/// The Party who specifies this Item.
    #[serde(default, rename = "InformationContentProviderParty")]
    pub information_content_provider_party: Option<crate::Party>,
/// A region (not country) of origin of this item.
    #[serde(default, rename = "OriginAddress")]
    pub origin_address: Vec<crate::Address>,
/// A trackable, unique instantiation of this item.
    #[serde(default, rename = "ItemInstance")]
    pub item_instance: Vec<ItemInstance>,
/// A certificate associated with this item.
    #[serde(default, rename = "Certificate")]
    pub certificate: Vec<crate::Certificate>,
/// One or more environmental certificatations issued for this item.
    #[serde(default, rename = "EnvironmentalCertificate")]
    pub environmental_certificate: Vec<crate::Certificate>,
/// One of the measurable dimensions (length, mass, weight, or volume) of this item.
    #[serde(default, rename = "Dimension")]
    pub dimension: Vec<crate::Dimension>,
/// One or more environmental emissions of this item.
    #[serde(default, rename = "EnvironmentalEmission")]
    pub environmental_emission: Vec<crate::EnvironmentalEmission>,
/// The Circularity Profile of this Item
    #[serde(default, rename = "CircularityProfile")]
    pub circularity_profile: Option<crate::CircularityProfile>,
}
