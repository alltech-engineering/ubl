#[derive(Debug, Deserialize, Serialize)]
pub struct Shipment {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "ShippingPriorityLevelCode")]
    pub shipping_priority_level_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "HandlingCode")]
    pub handling_code: Vec<super::cct::CodeType>,
    #[serde(default, rename = "HandlingInstructions")]
    pub handling_instructions: Vec<super::cct::TextType>,
    #[serde(default, rename = "Information")]
    pub information: Vec<super::cct::TextType>,
    #[serde(default, rename = "GrossWeightMeasure")]
    pub gross_weight_measure: Option<super::cct::MeasureType>,
    #[serde(default, rename = "NetWeightMeasure")]
    pub net_weight_measure: Option<super::cct::MeasureType>,
    #[serde(default, rename = "NetNetWeightMeasure")]
    pub net_net_weight_measure: Option<super::cct::MeasureType>,
    #[serde(default, rename = "GrossVolumeMeasure")]
    pub gross_volume_measure: Option<super::cct::MeasureType>,
    #[serde(default, rename = "NetVolumeMeasure")]
    pub net_volume_measure: Option<super::cct::MeasureType>,
    #[serde(default, rename = "TotalGoodsItemQuantity")]
    pub total_goods_item_quantity: Option<super::cct::QuantityType>,
    #[serde(default, rename = "TotalTransportHandlingUnitQuantity")]
    pub total_transport_handling_unit_quantity: Option<super::cct::QuantityType>,
    #[serde(default, rename = "InsuranceValueAmount")]
    pub insurance_value_amount: Option<super::cct::AmountType>,
    #[serde(default, rename = "DeclaredCustomsValueAmount")]
    pub declared_customs_value_amount: Option<super::cct::AmountType>,
    #[serde(default, rename = "DeclaredForCarriageValueAmount")]
    pub declared_for_carriage_value_amount: Option<super::cct::AmountType>,
    #[serde(default, rename = "DeclaredStatisticsValueAmount")]
    pub declared_statistics_value_amount: Option<super::cct::AmountType>,
    #[serde(default, rename = "FreeOnBoardValueAmount")]
    pub free_on_board_value_amount: Option<super::cct::AmountType>,
    #[serde(default, rename = "SpecialInstructions")]
    pub special_instructions: Vec<super::cct::TextType>,
    #[serde(default, rename = "DeliveryInstructions")]
    pub delivery_instructions: Vec<super::cct::TextType>,
    #[serde(default, rename = "SplitConsignmentIndicator")]
    pub split_consignment_indicator: Option<super::udt::IndicatorType>,
    #[serde(default, rename = "ConsignmentQuantity")]
    pub consignment_quantity: Option<super::cct::QuantityType>,
    #[serde(default, rename = "Consignment")]
    pub consignment: Vec<Consignment>,
    #[serde(default, rename = "GoodsItem")]
    pub goods_item: Vec<GoodsItem>,
    #[serde(default, rename = "ShipmentStage")]
    pub shipment_stage: Vec<ShipmentStage>,
    #[serde(default, rename = "Delivery")]
    pub delivery: Option<Box<Delivery>>,
    #[serde(default, rename = "TransportHandlingUnit")]
    pub transport_handling_unit: Vec<TransportHandlingUnit>,
    #[serde(default, rename = "ReturnAddress")]
    pub return_address: Option<Address>,
    #[serde(default, rename = "OriginAddress")]
    pub origin_address: Option<Address>,
    #[serde(default, rename = "FirstArrivalPortLocation")]
    pub first_arrival_port_location: Option<Location>,
    #[serde(default, rename = "LastExitPortLocation")]
    pub last_exit_port_location: Option<Location>,
    #[serde(default, rename = "ExportCountry")]
    pub export_country: Option<Country>,
    #[serde(default, rename = "FreightAllowanceCharge")]
    pub freight_allowance_charge: Vec<AllowanceCharge>,
    #[serde(default, rename = "InsurancePolicy")]
    pub insurance_policy: Vec<InsurancePolicy>,
}
