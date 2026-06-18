#[derive(Debug, Deserialize, Serialize)]
pub struct ProcurementProject {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "Name")]
    pub name: Vec<super::cct::TextType>,
    #[serde(default, rename = "Description")]
    pub description: Vec<super::cct::TextType>,
    #[serde(default, rename = "ProcurementTypeCode")]
    pub procurement_type_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "ProcurementSubTypeCode")]
    pub procurement_sub_type_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "QualityControlCode")]
    pub quality_control_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "RequiredFeeAmount")]
    pub required_fee_amount: Option<super::cct::AmountType>,
    #[serde(default, rename = "FeeDescription")]
    pub fee_description: Vec<super::cct::TextType>,
    #[serde(default, rename = "RequestedDeliveryDate")]
    pub requested_delivery_date: Option<super::udt::DateTimeType>,
    #[serde(default, rename = "EstimatedOverallContractQuantity")]
    pub estimated_overall_contract_quantity: Option<super::cct::QuantityType>,
    #[serde(default, rename = "Note")]
    pub note: Vec<super::cct::TextType>,
    #[serde(default, rename = "SMESuitableIndicator")]
    pub sme_suitable_indicator: Option<super::udt::IndicatorType>,
    #[serde(default, rename = "ProcurementAdditionalType")]
    pub procurement_additional_type: Vec<ProcurementAdditionalType>,
    #[serde(default, rename = "RequestedTenderTotal")]
    pub requested_tender_total: Option<RequestedTenderTotal>,
    #[serde(default, rename = "MainCommodityClassification")]
    pub main_commodity_classification: Vec<CommodityClassification>,
    #[serde(default, rename = "AdditionalCommodityClassification")]
    pub additional_commodity_classification: Vec<CommodityClassification>,
    #[serde(default, rename = "RealizedLocation")]
    pub realized_location: Vec<Location>,
    #[serde(default, rename = "PlannedPeriod")]
    pub planned_period: Option<Period>,
    #[serde(default, rename = "ContractExtension")]
    pub contract_extension: Option<ContractExtension>,
    #[serde(default, rename = "RequestForTenderLine")]
    pub request_for_tender_line: Vec<RequestForTenderLine>,
}
