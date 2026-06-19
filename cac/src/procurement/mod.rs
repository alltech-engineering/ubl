use serde::{Deserialize, Serialize};


include!("project_lot.rs");
include!("project_lot_reference.rs");
include!("additional.rs");

#[derive(Debug, Deserialize, Serialize)]
/// A class to describe a project to procure goods, works, or services.
///
/// UBL Dictionary Entry Name: `Procurement Project. Details`
///
/// Generated from XSD type `ProcurementProjectType`.
pub struct ProcurementProject {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// An identifier for this procurement project.
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
/// A name of this procurement project.
    #[serde(default, rename = "Name")]
    pub name: Vec<cct::Text>,
/// Text describing this procurement project.
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
/// A code signifying the type of procurement project (e.g., goods, works, services).
    #[serde(default, rename = "ProcurementTypeCode")]
    pub procurement_type_code: Option<cct::Code>,
/// A code signifying the subcategory of the type of work for this project (e.g., land surveying, IT
/// consulting).
    #[serde(default, rename = "ProcurementSubTypeCode")]
    pub procurement_sub_type_code: Option<cct::Code>,
/// The indication of whether or not the control quality is included in the works project.
    #[serde(default, rename = "QualityControlCode")]
    pub quality_control_code: Option<cct::Code>,
/// The amount of the reimbursement fee for concession procurement projects.
    #[serde(default, rename = "RequiredFeeAmount")]
    pub required_fee_amount: Option<cct::Amount>,
/// Text describing the reimbursement fee for concession procurement projects.
    #[serde(default, rename = "FeeDescription")]
    pub fee_description: Vec<cct::Text>,
/// The requested delivery date for this procurement project.
    #[serde(default, rename = "RequestedDeliveryDate")]
    pub requested_delivery_date: Option<udt::DateTime>,
/// The estimated overall quantity for this procurement project.
    #[serde(default, rename = "EstimatedOverallContractQuantity")]
    pub estimated_overall_contract_quantity: Option<cct::Quantity>,
/// Free-form text applying to the Procurement Project. This element may contain additional information
/// about the lot/contract that is not contained explicitly in another structure.
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
/// Suitable for Small- and Medium-sized Enterprises. This element specifies that the buyer accepts the
/// risks associated of contracting with SMEs.
    #[serde(default, rename = "SMESuitableIndicator")]
    pub sme_suitable_indicator: Option<udt::Indicator>,
/// An association to additional procurement type.
    #[serde(default, rename = "ProcurementAdditionalType")]
    pub procurement_additional_type: Vec<ProcurementAdditional>,
/// Budget monetary amounts for the project as whole.
    #[serde(default, rename = "RequestedTenderTotal")]
    pub requested_tender_total: Option<crate::RequestedTenderTotal>,
/// An association to the main classification category for the deliverable requested.
    #[serde(default, rename = "MainCommodityClassification")]
    pub main_commodity_classification: Vec<crate::CommodityClassification>,
/// An association to additional classification categories for the deliverable requested.
    #[serde(default, rename = "AdditionalCommodityClassification")]
    pub additional_commodity_classification: Vec<crate::CommodityClassification>,
/// A place where this procurement project will be physically realized.
    #[serde(default, rename = "RealizedLocation")]
    pub realized_location: Vec<crate::Location>,
/// The period during which this procurement project is planned to take place.
    #[serde(default, rename = "PlannedPeriod")]
    pub planned_period: Option<crate::Period>,
/// The contract extension for this tendering process.
    #[serde(default, rename = "ContractExtension")]
    pub contract_extension: Option<crate::ContractExtension>,
/// A good or service this project is intended to procure.
    #[serde(default, rename = "RequestForTenderLine")]
    pub request_for_tender_line: Vec<crate::RequestForTenderLine>,
}
