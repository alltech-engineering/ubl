#[derive(Debug, Deserialize, Serialize)]
pub struct GoodsItem {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "SequenceNumberID")]
    pub sequence_number_id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "Description")]
    pub description: Vec<super::cct::TextType>,
    #[serde(default, rename = "HazardousRiskIndicator")]
    pub hazardous_risk_indicator: Option<super::udt::IndicatorType>,
    #[serde(default, rename = "DeclaredCustomsValueAmount")]
    pub declared_customs_value_amount: Option<super::cct::AmountType>,
    #[serde(default, rename = "DeclaredForCarriageValueAmount")]
    pub declared_for_carriage_value_amount: Option<super::cct::AmountType>,
    #[serde(default, rename = "DeclaredStatisticsValueAmount")]
    pub declared_statistics_value_amount: Option<super::cct::AmountType>,
    #[serde(default, rename = "FreeOnBoardValueAmount")]
    pub free_on_board_value_amount: Option<super::cct::AmountType>,
    #[serde(default, rename = "InsuranceValueAmount")]
    pub insurance_value_amount: Option<super::cct::AmountType>,
    #[serde(default, rename = "ValueAmount")]
    pub value_amount: Option<super::cct::AmountType>,
    #[serde(default, rename = "GrossWeightMeasure")]
    pub gross_weight_measure: Option<super::cct::MeasureType>,
    #[serde(default, rename = "NetWeightMeasure")]
    pub net_weight_measure: Option<super::cct::MeasureType>,
    #[serde(default, rename = "NetNetWeightMeasure")]
    pub net_net_weight_measure: Option<super::cct::MeasureType>,
    #[serde(default, rename = "ChargeableWeightMeasure")]
    pub chargeable_weight_measure: Option<super::cct::MeasureType>,
    #[serde(default, rename = "GrossVolumeMeasure")]
    pub gross_volume_measure: Option<super::cct::MeasureType>,
    #[serde(default, rename = "NetVolumeMeasure")]
    pub net_volume_measure: Option<super::cct::MeasureType>,
    #[serde(default, rename = "Quantity")]
    pub quantity: Option<super::cct::QuantityType>,
    #[serde(default, rename = "PreferenceCriterionCode")]
    pub preference_criterion_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "RequiredCustomsID")]
    pub required_customs_id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "CustomsStatusCode")]
    pub customs_status_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "CustomsProcedureCode")]
    pub customs_procedure_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "CustomsTariffQuantity")]
    pub customs_tariff_quantity: Option<super::cct::QuantityType>,
    #[serde(default, rename = "CustomsImportClassifiedIndicator")]
    pub customs_import_classified_indicator: Option<super::udt::IndicatorType>,
    #[serde(default, rename = "ChargeableQuantity")]
    pub chargeable_quantity: Option<super::cct::QuantityType>,
    #[serde(default, rename = "ReturnableQuantity")]
    pub returnable_quantity: Option<super::cct::QuantityType>,
    #[serde(default, rename = "TraceID")]
    pub trace_id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "Item")]
    pub item: Vec<Item>,
    #[serde(default, rename = "GoodsItemContainer")]
    pub goods_item_container: Vec<GoodsItemContainer>,
    #[serde(default, rename = "FreightAllowanceCharge")]
    pub freight_allowance_charge: Vec<AllowanceCharge>,
    #[serde(default, rename = "InvoiceLine")]
    pub invoice_line: Vec<InvoiceLine>,
    #[serde(default, rename = "OrderLineReference")]
    pub order_line_reference: Vec<OrderLineReference>,
    #[serde(default, rename = "DespatchLineReference")]
    pub despatch_line_reference: Option<LineReference>,
    #[serde(default, rename = "ReceiptLineReference")]
    pub receipt_line_reference: Option<LineReference>,
    #[serde(default, rename = "Temperature")]
    pub temperature: Vec<Temperature>,
    #[serde(default, rename = "ContainedGoodsItem")]
    pub contained_goods_item: Vec<GoodsItem>,
    #[serde(default, rename = "OriginAddress")]
    pub origin_address: Option<Address>,
    #[serde(default, rename = "Delivery")]
    pub delivery: Option<Delivery>,
    #[serde(default, rename = "Pickup")]
    pub pickup: Option<Pickup>,
    #[serde(default, rename = "Despatch")]
    pub despatch: Option<Despatch>,
    #[serde(default, rename = "BondedWarehouseLocation")]
    pub bonded_warehouse_location: Option<Location>,
    #[serde(default, rename = "MeasurementDimension")]
    pub measurement_dimension: Vec<Dimension>,
    #[serde(default, rename = "ContainingPackage")]
    pub containing_package: Vec<Package>,
    #[serde(default, rename = "ShipmentDocumentReference")]
    pub shipment_document_reference: Option<DocumentReference>,
    #[serde(default, rename = "AdditionalDocumentReference")]
    pub additional_document_reference: Vec<DocumentReference>,
    #[serde(default, rename = "MinimumTemperature")]
    pub minimum_temperature: Option<Temperature>,
    #[serde(default, rename = "MaximumTemperature")]
    pub maximum_temperature: Option<Temperature>,
    #[serde(default, rename = "InsurancePolicy")]
    pub insurance_policy: Vec<InsurancePolicy>,
    #[serde(default, rename = "EnergyConsumptionAllocation")]
    pub energy_consumption_allocation: Vec<EnergyConsumptionAllocation>,
}
