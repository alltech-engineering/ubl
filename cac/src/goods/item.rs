#[derive(Debug, Deserialize, Serialize)]
/// A class to describe a separately identifiable quantity of goods of a single product type.
///
/// UBL Dictionary Entry Name: `Goods Item. Details`
///
/// Generated from XSD type `GoodsItemType`.
pub struct GoodsItem {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
/// An identifier for this goods item.
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
/// A sequence number differentiating a specific goods item within a consignment.
    #[serde(default, rename = "SequenceNumberID")]
    pub sequence_number_id: Option<cct::Identifier>,
/// Text describing this goods item to identify it for customs, statistical, or transport purposes.
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
/// An indication that the transported goods item is subject to an international regulation concerning
/// the carriage of dangerous goods (true) or not (false).
    #[serde(default, rename = "HazardousRiskIndicator")]
    pub hazardous_risk_indicator: Option<udt::Indicator>,
/// The total declared value for customs purposes of the goods item.
    #[serde(default, rename = "DeclaredCustomsValueAmount")]
    pub declared_customs_value_amount: Option<cct::Amount>,
/// The value of this goods item, declared by the shipper or his agent solely for the purpose of varying
/// the carrier's level of liability from that provided in the contract of carriage, in case of loss or
/// damage to goods or delayed delivery.
    #[serde(default, rename = "DeclaredForCarriageValueAmount")]
    pub declared_for_carriage_value_amount: Option<cct::Amount>,
/// The total declared value of all the goods items in the same consignment with this goods item that
/// have the same statistical heading.
    #[serde(default, rename = "DeclaredStatisticsValueAmount")]
    pub declared_statistics_value_amount: Option<cct::Amount>,
/// The monetary amount that has to be or has been paid as calculated under the applicable trade
/// delivery.
    #[serde(default, rename = "FreeOnBoardValueAmount")]
    pub free_on_board_value_amount: Option<cct::Amount>,
/// The amount covered by insurance for this goods item.
    #[serde(default, rename = "InsuranceValueAmount")]
    pub insurance_value_amount: Option<cct::Amount>,
/// The amount on which a duty, tax, or fee will be assessed.
    #[serde(default, rename = "ValueAmount")]
    pub value_amount: Option<cct::Amount>,
/// The weight of this goods item, including packing and packaging but excluding the carrier's
/// equipment.
    #[serde(default, rename = "GrossWeightMeasure")]
    pub gross_weight_measure: Option<cct::Measure>,
/// The weight of this goods item, excluding packing but including packaging that normally accompanies
/// the goods.
    #[serde(default, rename = "NetWeightMeasure")]
    pub net_weight_measure: Option<cct::Measure>,
/// The total weight of this goods item, excluding all packing and packaging.
    #[serde(default, rename = "NetNetWeightMeasure")]
    pub net_net_weight_measure: Option<cct::Measure>,
/// The weight on which a charge is to be based.
    #[serde(default, rename = "ChargeableWeightMeasure")]
    pub chargeable_weight_measure: Option<cct::Measure>,
/// The volume of this goods item, normally calculated by multiplying its maximum length, width, and
/// height.
    #[serde(default, rename = "GrossVolumeMeasure")]
    pub gross_volume_measure: Option<cct::Measure>,
/// The volume contained by a goods item, excluding the volume of any packaging material.
    #[serde(default, rename = "NetVolumeMeasure")]
    pub net_volume_measure: Option<cct::Measure>,
/// The number of units making up this goods item.
    #[serde(default, rename = "Quantity")]
    pub quantity: Option<cct::Quantity>,
/// A code signifying the treatment preference for this goods item according to international trading
/// agreements.
    #[serde(default, rename = "PreferenceCriterionCode")]
    pub preference_criterion_code: Option<cct::Code>,
/// An identifier for a set of tariff codes required to specify a type of goods for customs, transport,
/// statistical, or other regulatory purposes.
    #[serde(default, rename = "RequiredCustomsID")]
    pub required_customs_id: Option<cct::Identifier>,
/// A code assigned by customs to signify the status of this goods item.
    #[serde(default, rename = "CustomsStatusCode")]
    pub customs_status_code: Option<cct::Code>,
/// A code assigned by customs to signifying the customs procedure applied to this Goods Item.
    #[serde(default, rename = "CustomsProcedureCode")]
    pub customs_procedure_code: Option<cct::Code>,
/// Quantity of the units in this goods item as required by customs for tariff, statistical, or fiscal
/// purposes.
    #[serde(default, rename = "CustomsTariffQuantity")]
    pub customs_tariff_quantity: Option<cct::Quantity>,
/// An indicator that this goods item has been classified for import by customs (true) or not (false).
    #[serde(default, rename = "CustomsImportClassifiedIndicator")]
    pub customs_import_classified_indicator: Option<udt::Indicator>,
/// The number of units in the goods item to which charges apply.
    #[serde(default, rename = "ChargeableQuantity")]
    pub chargeable_quantity: Option<cct::Quantity>,
/// The number of units in the goods item that may be returned.
    #[serde(default, rename = "ReturnableQuantity")]
    pub returnable_quantity: Option<cct::Quantity>,
/// An identifier for use in tracing this goods item, such as the EPC number used in RFID.
    #[serde(default, rename = "TraceID")]
    pub trace_id: Option<cct::Identifier>,
/// Product information relating to a goods item.
    #[serde(default, rename = "Item")]
    pub item: Vec<crate::Item>,
/// The transporting of a goods item in a unit of transport equipment (e.g., container).
    #[serde(default, rename = "GoodsItemContainer")]
    pub goods_item_container: Vec<GoodsItemContainer>,
/// A cost incurred by the shipper in moving goods, by whatever means, from one place to another under
/// the terms of the contract of carriage. In addition to transport costs, this may include such
/// elements as packing, documentation, loading, unloading, and insurance to the extent that they relate
/// to the freight costs.
    #[serde(default, rename = "FreightAllowanceCharge")]
    pub freight_allowance_charge: Vec<crate::AllowanceCharge>,
/// Information about an invoice line relating to this goods item.
    #[serde(default, rename = "InvoiceLine")]
    pub invoice_line: Vec<crate::InvoiceLine>,
/// A reference to an order line associated with this goods item.
    #[serde(default, rename = "OrderLineReference")]
    pub order_line_reference: Vec<crate::OrderLineReference>,
/// A reference to the despatch line associated with this goods item.
    #[serde(default, rename = "DespatchLineReference")]
    pub despatch_line_reference: Option<crate::LineReference>,
/// A reference to the receipt line associated with this goods item.
    #[serde(default, rename = "ReceiptLineReference")]
    pub receipt_line_reference: Option<crate::LineReference>,
/// The temperature of the goods item.
    #[serde(default, rename = "Temperature")]
    pub temperature: Vec<crate::Temperature>,
/// A goods item contained in this goods item.
    #[serde(default, rename = "ContainedGoodsItem")]
    pub contained_goods_item: Vec<GoodsItem>,
/// The region in which the goods have been produced or manufactured, according to criteria laid down
/// for the purposes of application of the customs tariff, or of quantitative restrictions, or of any
/// other measure related to trade.
    #[serde(default, rename = "OriginAddress")]
    pub origin_address: Option<crate::Address>,
/// The delivery of this goods item.
    #[serde(default, rename = "Delivery")]
    pub delivery: Option<crate::Delivery>,
/// The pickup of this goods item.
    #[serde(default, rename = "Pickup")]
    pub pickup: Option<crate::Pickup>,
/// The despatch of this goods item.
    #[serde(default, rename = "Despatch")]
    pub despatch: Option<crate::Despatch>,
/// The location of the bonded warehouse where this goods item is temporarily stored.
    #[serde(default, rename = "BondedWarehouseLocation")]
    pub bonded_warehouse_location: Option<crate::Location>,
/// A measurable dimension (length, mass, weight, or volume) of this goods item.
    #[serde(default, rename = "MeasurementDimension")]
    pub measurement_dimension: Vec<crate::Dimension>,
/// A package containing this goods item.
    #[serde(default, rename = "ContainingPackage")]
    pub containing_package: Vec<crate::Package>,
/// A reference to a shipping document associated with this goods item.
    #[serde(default, rename = "ShipmentDocumentReference")]
    pub shipment_document_reference: Option<crate::DocumentReference>,
/// A reference to an additional document associated with this goods item.
    #[serde(default, rename = "AdditionalDocumentReference")]
    pub additional_document_reference: Vec<crate::DocumentReference>,
/// Information about minimum temperature.
    #[serde(default, rename = "MinimumTemperature")]
    pub minimum_temperature: Option<crate::Temperature>,
/// Information about maximum temperature.
    #[serde(default, rename = "MaximumTemperature")]
    pub maximum_temperature: Option<crate::Temperature>,
/// One or more Insurance Policies that apply to this Goods Item.
    #[serde(default, rename = "InsurancePolicy")]
    pub insurance_policy: Vec<crate::InsurancePolicy>,
/// An allocation of energy consumption and associated emissions attributable to the transport of this
/// goods item.
    #[serde(default, rename = "EnergyConsumptionAllocation")]
    pub energy_consumption_allocation: Vec<crate::EnergyConsumptionAllocation>,
}
