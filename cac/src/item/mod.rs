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
pub struct Item {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
    #[serde(default, rename = "PackQuantity")]
    pub pack_quantity: Option<cct::Quantity>,
    #[serde(default, rename = "PackSizeNumeric")]
    pub pack_size_numeric: Option<cct::Numeric>,
    #[serde(default, rename = "CatalogueIndicator")]
    pub catalogue_indicator: Option<udt::Indicator>,
    #[serde(default, rename = "Name")]
    pub name: Vec<cct::Text>,
    #[serde(default, rename = "ItemTypeCode")]
    pub item_type_code: Option<cct::Code>,
    #[serde(default, rename = "HazardousRiskIndicator")]
    pub hazardous_risk_indicator: Option<udt::Indicator>,
    #[serde(default, rename = "AdditionalInformation")]
    pub additional_information: Vec<cct::Text>,
    #[serde(default, rename = "Keyword")]
    pub keyword: Vec<cct::Text>,
    #[serde(default, rename = "BrandName")]
    pub brand_name: Vec<cct::Text>,
    #[serde(default, rename = "ModelName")]
    pub model_name: Vec<cct::Text>,
    #[serde(default, rename = "WarrantyInformation")]
    pub warranty_information: Vec<cct::Text>,
    #[serde(default, rename = "LifecycleStageCode")]
    pub lifecycle_stage_code: Option<cct::Code>,
    #[serde(default, rename = "LifecycleStageDescription")]
    pub lifecycle_stage_description: Vec<cct::Text>,
    #[serde(default, rename = "BuyersItemIdentification")]
    pub buyers_item_identification: Option<ItemIdentification>,
    #[serde(default, rename = "SellersItemIdentification")]
    pub sellers_item_identification: Option<ItemIdentification>,
    #[serde(default, rename = "ManufacturersItemIdentification")]
    pub manufacturers_item_identification: Vec<ItemIdentification>,
    #[serde(default, rename = "StandardItemIdentification")]
    pub standard_item_identification: Option<ItemIdentification>,
    #[serde(default, rename = "CatalogueItemIdentification")]
    pub catalogue_item_identification: Option<ItemIdentification>,
    #[serde(default, rename = "AdditionalItemIdentification")]
    pub additional_item_identification: Vec<ItemIdentification>,
    #[serde(default, rename = "CatalogueDocumentReference")]
    pub catalogue_document_reference: Option<crate::DocumentReference>,
    #[serde(default, rename = "ItemSpecificationDocumentReference")]
    pub item_specification_document_reference: Vec<crate::DocumentReference>,
    #[serde(default, rename = "OriginCountry")]
    pub origin_country: Option<crate::Country>,
    #[serde(default, rename = "CommodityClassification")]
    pub commodity_classification: Vec<crate::CommodityClassification>,
    #[serde(default, rename = "TransactionConditions")]
    pub transaction_conditions: Vec<crate::TransactionConditions>,
    #[serde(default, rename = "HazardousItem")]
    pub hazardous_item: Vec<crate::HazardousItem>,
    #[serde(default, rename = "ClassifiedTaxCategory")]
    pub classified_tax_category: Vec<crate::TaxCategory>,
    #[serde(default, rename = "AdditionalItemProperty")]
    pub additional_item_property: Vec<ItemProperty>,
    #[serde(default, rename = "ManufacturerParty")]
    pub manufacturer_party: Vec<crate::Party>,
    #[serde(default, rename = "InformationContentProviderParty")]
    pub information_content_provider_party: Option<crate::Party>,
    #[serde(default, rename = "OriginAddress")]
    pub origin_address: Vec<crate::Address>,
    #[serde(default, rename = "ItemInstance")]
    pub item_instance: Vec<ItemInstance>,
    #[serde(default, rename = "Certificate")]
    pub certificate: Vec<crate::Certificate>,
    #[serde(default, rename = "EnvironmentalCertificate")]
    pub environmental_certificate: Vec<crate::Certificate>,
    #[serde(default, rename = "Dimension")]
    pub dimension: Vec<crate::Dimension>,
    #[serde(default, rename = "EnvironmentalEmission")]
    pub environmental_emission: Vec<crate::EnvironmentalEmission>,
    #[serde(default, rename = "CircularityProfile")]
    pub circularity_profile: Option<crate::CircularityProfile>,
}
