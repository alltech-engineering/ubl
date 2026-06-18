#[derive(Debug, Deserialize, Serialize)]
pub struct LineItem {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(rename = "ID")]
    pub id: super::cct::IdentifierType,
    #[serde(default, rename = "SalesOrderID")]
    pub sales_order_id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "UUID")]
    pub uuid: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "Note")]
    pub note: Vec<super::cct::TextType>,
    #[serde(default, rename = "LineStatusCode")]
    pub line_status_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "Quantity")]
    pub quantity: Option<super::cct::QuantityType>,
    #[serde(default, rename = "LineExtensionAmount")]
    pub line_extension_amount: Option<super::cct::AmountType>,
    #[serde(default, rename = "TaxInclusiveLineExtensionAmount")]
    pub tax_inclusive_line_extension_amount: Option<super::cct::AmountType>,
    #[serde(default, rename = "TotalTaxAmount")]
    pub total_tax_amount: Option<super::cct::AmountType>,
    #[serde(default, rename = "MinimumQuantity")]
    pub minimum_quantity: Option<super::cct::QuantityType>,
    #[serde(default, rename = "MaximumQuantity")]
    pub maximum_quantity: Option<super::cct::QuantityType>,
    #[serde(default, rename = "MinimumBackorderQuantity")]
    pub minimum_backorder_quantity: Option<super::cct::QuantityType>,
    #[serde(default, rename = "MaximumBackorderQuantity")]
    pub maximum_backorder_quantity: Option<super::cct::QuantityType>,
    #[serde(default, rename = "InspectionMethodCode")]
    pub inspection_method_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "PartialDeliveryIndicator")]
    pub partial_delivery_indicator: Option<super::udt::IndicatorType>,
    #[serde(default, rename = "BackOrderAllowedIndicator")]
    pub back_order_allowed_indicator: Option<super::udt::IndicatorType>,
    #[serde(default, rename = "AccountingCostCode")]
    pub accounting_cost_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "AccountingCost")]
    pub accounting_cost: Option<super::cct::TextType>,
    #[serde(default, rename = "WarrantyInformation")]
    pub warranty_information: Vec<super::cct::TextType>,
    #[serde(default, rename = "Delivery")]
    pub delivery: Vec<Delivery>,
    #[serde(default, rename = "DeliveryTerms")]
    pub delivery_terms: Option<DeliveryTerms>,
    #[serde(default, rename = "OriginatorParty")]
    pub originator_party: Option<Party>,
    #[serde(default, rename = "BeneficiaryParty")]
    pub beneficiary_party: Vec<Party>,
    #[serde(default, rename = "OrderedShipment")]
    pub ordered_shipment: Vec<OrderedShipment>,
    #[serde(default, rename = "PricingReference")]
    pub pricing_reference: Option<PricingReference>,
    #[serde(default, rename = "AllowanceCharge")]
    pub allowance_charge: Vec<AllowanceCharge>,
    #[serde(default, rename = "Price")]
    pub price: Option<Price>,
    #[serde(rename = "Item")]
    pub item: Item,
    #[serde(default, rename = "SubLineItem")]
    pub sub_line_item: Vec<LineItem>,
    #[serde(default, rename = "WarrantyValidityPeriod")]
    pub warranty_validity_period: Option<Period>,
    #[serde(default, rename = "WarrantyParty")]
    pub warranty_party: Option<Party>,
    #[serde(default, rename = "TaxTotal")]
    pub tax_total: Vec<TaxTotal>,
    #[serde(default, rename = "ItemPriceExtension")]
    pub item_price_extension: Option<PriceExtension>,
    #[serde(default, rename = "LineReference")]
    pub line_reference: Vec<LineReference>,
}
