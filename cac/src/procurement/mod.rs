use serde::{Deserialize, Serialize};


include!("project_lot.rs");
include!("project_lot_reference.rs");
include!("additional_type.rs");

#[derive(Debug, Deserialize, Serialize)]
pub struct ProcurementProject {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
    #[serde(default, rename = "Name")]
    pub name: Vec<cct::Text>,
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
    #[serde(default, rename = "ProcurementTypeCode")]
    pub procurement_type_code: Option<cct::Code>,
    #[serde(default, rename = "ProcurementSubTypeCode")]
    pub procurement_sub_type_code: Option<cct::Code>,
    #[serde(default, rename = "QualityControlCode")]
    pub quality_control_code: Option<cct::Code>,
    #[serde(default, rename = "RequiredFeeAmount")]
    pub required_fee_amount: Option<cct::Amount>,
    #[serde(default, rename = "FeeDescription")]
    pub fee_description: Vec<cct::Text>,
    #[serde(default, rename = "RequestedDeliveryDate")]
    pub requested_delivery_date: Option<udt::DateTime>,
    #[serde(default, rename = "EstimatedOverallContractQuantity")]
    pub estimated_overall_contract_quantity: Option<cct::Quantity>,
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
    #[serde(default, rename = "SMESuitableIndicator")]
    pub sme_suitable_indicator: Option<udt::Indicator>,
    #[serde(default, rename = "ProcurementAdditionalType")]
    pub procurement_additional_type: Vec<ProcurementAdditionalType>,
    #[serde(default, rename = "RequestedTenderTotal")]
    pub requested_tender_total: Option<crate::RequestedTenderTotal>,
    #[serde(default, rename = "MainCommodityClassification")]
    pub main_commodity_classification: Vec<crate::CommodityClassification>,
    #[serde(default, rename = "AdditionalCommodityClassification")]
    pub additional_commodity_classification: Vec<crate::CommodityClassification>,
    #[serde(default, rename = "RealizedLocation")]
    pub realized_location: Vec<crate::Location>,
    #[serde(default, rename = "PlannedPeriod")]
    pub planned_period: Option<crate::Period>,
    #[serde(default, rename = "ContractExtension")]
    pub contract_extension: Option<crate::ContractExtension>,
    #[serde(default, rename = "RequestForTenderLine")]
    pub request_for_tender_line: Vec<crate::RequestForTenderLine>,
}
