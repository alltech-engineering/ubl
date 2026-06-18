#[derive(Debug, Deserialize, Serialize)]
pub struct RequestForTenderLine {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "UUID")]
    pub uuid: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "Note")]
    pub note: Vec<super::cct::TextType>,
    #[serde(default, rename = "Quantity")]
    pub quantity: Option<super::cct::QuantityType>,
    #[serde(default, rename = "MinimumQuantity")]
    pub minimum_quantity: Option<super::cct::QuantityType>,
    #[serde(default, rename = "MaximumQuantity")]
    pub maximum_quantity: Option<super::cct::QuantityType>,
    #[serde(default, rename = "TaxIncludedIndicator")]
    pub tax_included_indicator: Option<super::udt::IndicatorType>,
    #[serde(default, rename = "MinimumAmount")]
    pub minimum_amount: Option<super::cct::AmountType>,
    #[serde(default, rename = "MaximumAmount")]
    pub maximum_amount: Option<super::cct::AmountType>,
    #[serde(default, rename = "EstimatedAmount")]
    pub estimated_amount: Option<super::cct::AmountType>,
    #[serde(default, rename = "DocumentReference")]
    pub document_reference: Vec<DocumentReference>,
    #[serde(default, rename = "DeliveryPeriod")]
    pub delivery_period: Vec<Period>,
    #[serde(default, rename = "RequiredItemLocationQuantity")]
    pub required_item_location_quantity: Vec<ItemLocationQuantity>,
    #[serde(default, rename = "WarrantyValidityPeriod")]
    pub warranty_validity_period: Option<Period>,
    #[serde(rename = "Item")]
    pub item: Item,
    #[serde(default, rename = "SubRequestForTenderLine")]
    pub sub_request_for_tender_line: Vec<RequestForTenderLine>,
}
