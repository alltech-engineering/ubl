#[derive(Debug, Deserialize, Serialize)]
pub struct Item {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "Description")]
    pub description: Vec<super::cct::TextType>,
    #[serde(default, rename = "PackQuantity")]
    pub pack_quantity: Option<super::cct::QuantityType>,
    #[serde(default, rename = "PackSizeNumeric")]
    pub pack_size_numeric: Option<super::cct::NumericType>,
    #[serde(default, rename = "CatalogueIndicator")]
    pub catalogue_indicator: Option<super::udt::IndicatorType>,
    #[serde(default, rename = "Name")]
    pub name: Vec<super::cct::TextType>,
    #[serde(default, rename = "ItemTypeCode")]
    pub item_type_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "HazardousRiskIndicator")]
    pub hazardous_risk_indicator: Option<super::udt::IndicatorType>,
    #[serde(default, rename = "AdditionalInformation")]
    pub additional_information: Vec<super::cct::TextType>,
    #[serde(default, rename = "Keyword")]
    pub keyword: Vec<super::cct::TextType>,
    #[serde(default, rename = "BrandName")]
    pub brand_name: Vec<super::cct::TextType>,
    #[serde(default, rename = "ModelName")]
    pub model_name: Vec<super::cct::TextType>,
    #[serde(default, rename = "WarrantyInformation")]
    pub warranty_information: Vec<super::cct::TextType>,
    #[serde(default, rename = "LifecycleStageCode")]
    pub lifecycle_stage_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "LifecycleStageDescription")]
    pub lifecycle_stage_description: Vec<super::cct::TextType>,
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
    pub catalogue_document_reference: Option<DocumentReference>,
    #[serde(default, rename = "ItemSpecificationDocumentReference")]
    pub item_specification_document_reference: Vec<DocumentReference>,
    #[serde(default, rename = "OriginCountry")]
    pub origin_country: Option<Country>,
    #[serde(default, rename = "CommodityClassification")]
    pub commodity_classification: Vec<CommodityClassification>,
    #[serde(default, rename = "TransactionConditions")]
    pub transaction_conditions: Vec<TransactionConditions>,
    #[serde(default, rename = "HazardousItem")]
    pub hazardous_item: Vec<HazardousItem>,
    #[serde(default, rename = "ClassifiedTaxCategory")]
    pub classified_tax_category: Vec<TaxCategory>,
    #[serde(default, rename = "AdditionalItemProperty")]
    pub additional_item_property: Vec<ItemProperty>,
    #[serde(default, rename = "ManufacturerParty")]
    pub manufacturer_party: Vec<Party>,
    #[serde(default, rename = "InformationContentProviderParty")]
    pub information_content_provider_party: Option<Party>,
    #[serde(default, rename = "OriginAddress")]
    pub origin_address: Vec<Address>,
    #[serde(default, rename = "ItemInstance")]
    pub item_instance: Vec<ItemInstance>,
    #[serde(default, rename = "Certificate")]
    pub certificate: Vec<Certificate>,
    #[serde(default, rename = "EnvironmentalCertificate")]
    pub environmental_certificate: Vec<Certificate>,
    #[serde(default, rename = "Dimension")]
    pub dimension: Vec<Dimension>,
    #[serde(default, rename = "EnvironmentalEmission")]
    pub environmental_emission: Vec<EnvironmentalEmission>,
    #[serde(default, rename = "CircularityProfile")]
    pub circularity_profile: Option<CircularityProfile>,
}
