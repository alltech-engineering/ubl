#[derive(Debug, Deserialize, Serialize)]
pub struct LineItem {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(rename = "ID")]
    pub id: cct::Identifier,
    #[serde(default, rename = "SalesOrderID")]
    pub sales_order_id: Option<cct::Identifier>,
    #[serde(default, rename = "UUID")]
    pub uuid: Option<cct::Identifier>,
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
    #[serde(default, rename = "LineStatusCode")]
    pub line_status_code: Option<cct::Code>,
    #[serde(default, rename = "Quantity")]
    pub quantity: Option<cct::Quantity>,
    #[serde(default, rename = "LineExtensionAmount")]
    pub line_extension_amount: Option<cct::Amount>,
    #[serde(default, rename = "TaxInclusiveLineExtensionAmount")]
    pub tax_inclusive_line_extension_amount: Option<cct::Amount>,
    #[serde(default, rename = "TotalTaxAmount")]
    pub total_tax_amount: Option<cct::Amount>,
    #[serde(default, rename = "MinimumQuantity")]
    pub minimum_quantity: Option<cct::Quantity>,
    #[serde(default, rename = "MaximumQuantity")]
    pub maximum_quantity: Option<cct::Quantity>,
    #[serde(default, rename = "MinimumBackorderQuantity")]
    pub minimum_backorder_quantity: Option<cct::Quantity>,
    #[serde(default, rename = "MaximumBackorderQuantity")]
    pub maximum_backorder_quantity: Option<cct::Quantity>,
    #[serde(default, rename = "InspectionMethodCode")]
    pub inspection_method_code: Option<cct::Code>,
    #[serde(default, rename = "PartialDeliveryIndicator")]
    pub partial_delivery_indicator: Option<udt::Indicator>,
    #[serde(default, rename = "BackOrderAllowedIndicator")]
    pub back_order_allowed_indicator: Option<udt::Indicator>,
    #[serde(default, rename = "AccountingCostCode")]
    pub accounting_cost_code: Option<cct::Code>,
    #[serde(default, rename = "AccountingCost")]
    pub accounting_cost: Option<cct::Text>,
    #[serde(default, rename = "WarrantyInformation")]
    pub warranty_information: Vec<cct::Text>,
    #[serde(default, rename = "Delivery")]
    pub delivery: Vec<crate::Delivery>,
    #[serde(default, rename = "DeliveryTerms")]
    pub delivery_terms: Option<crate::DeliveryTerms>,
    #[serde(default, rename = "OriginatorParty")]
    pub originator_party: Option<crate::Party>,
    #[serde(default, rename = "BeneficiaryParty")]
    pub beneficiary_party: Vec<crate::Party>,
    #[serde(default, rename = "OrderedShipment")]
    pub ordered_shipment: Vec<crate::OrderedShipment>,
    #[serde(default, rename = "PricingReference")]
    pub pricing_reference: Option<crate::PricingReference>,
    #[serde(default, rename = "AllowanceCharge")]
    pub allowance_charge: Vec<crate::AllowanceCharge>,
    #[serde(default, rename = "Price")]
    pub price: Option<crate::Price>,
    #[serde(rename = "Item")]
    pub item: crate::Item,
    #[serde(default, rename = "SubLineItem")]
    pub sub_line_item: Vec<LineItem>,
    #[serde(default, rename = "WarrantyValidityPeriod")]
    pub warranty_validity_period: Option<crate::Period>,
    #[serde(default, rename = "WarrantyParty")]
    pub warranty_party: Option<crate::Party>,
    #[serde(default, rename = "TaxTotal")]
    pub tax_total: Vec<crate::TaxTotal>,
    #[serde(default, rename = "ItemPriceExtension")]
    pub item_price_extension: Option<crate::PriceExtension>,
    #[serde(default, rename = "LineReference")]
    pub line_reference: Vec<LineReference>,
}
